#!/usr/bin/env python3
"""
政策监控系统 - 抓取脚本 V2 (HTML 版)
翰林院编制 - 中国政府政策监控

功能：
1. 直接抓取政府网站 HTML（绕过失效的 RSS）
2. 识别重大政策
3. 生成推送内容
4. 记录到 SQLite 数据库
"""

import requests
from bs4 import BeautifulSoup
import sqlite3
import yaml
import logging
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Optional

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# 请求头（模拟浏览器）
HEADERS = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
    'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
}


class PolicyCrawlerV2:
    """政策抓取器 V2 - HTML 版"""
    
    def __init__(self, config_path: str = "config.yaml"):
        """初始化抓取器"""
        self.config = self._load_config(config_path)
        self.db_path = self.config['storage']['database']
        self._init_database()
        
    def _load_config(self, config_path: str) -> Dict:
        """加载配置文件"""
        config_file = Path(__file__).parent / config_path
        with open(config_file, 'r', encoding='utf-8') as f:
            return yaml.safe_load(f)
    
    def _init_database(self):
        """初始化 SQLite 数据库"""
        db_file = Path(__file__).parent / self.db_path
        db_file.parent.mkdir(parents=True, exist_ok=True)
        
        conn = sqlite3.connect(str(db_file))
        cursor = conn.cursor()
        
        # 创建政策表
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS policies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                source TEXT,
                link TEXT UNIQUE,
                published TEXT,
                level TEXT,
                summary TEXT,
                pushed INTEGER DEFAULT 0,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        # 创建推送记录表
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS push_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                policy_id INTEGER,
                push_time TEXT,
                channel TEXT,
                FOREIGN KEY (policy_id) REFERENCES policies(id)
            )
        ''')
        
        conn.commit()
        conn.close()
        logger.info(f"数据库初始化完成：{db_file}")
    
    def fetch_gov_cn(self) -> List[Dict]:
        """抓取国务院政策文件库 (gov.cn)"""
        logger.info("抓取国务院政策文件库...")
        entries = []
        
        try:
            # 国务院最新政策
            url = "https://www.gov.cn/zhengce/zuixin.htm"
            response = requests.get(url, headers=HEADERS, timeout=30)
            response.encoding = 'utf-8'
            soup = BeautifulSoup(response.text, 'lxml')
            
            # 查找政策列表（根据实际 HTML 结构调整）
            links = soup.select('li a[href*="/zhengce/content/"]')
            
            for link in links[:20]:  # 最多取 20 条
                title = link.get_text(strip=True)
                href = link.get('href', '')
                
                if title and href:
                    full_url = f"https://www.gov.cn{href}" if href.startswith('/') else href
                    entries.append({
                        'title': title,
                        'link': full_url,
                        'published': datetime.now().isoformat(),
                        'summary': title[:200],  # 暂用标题作为摘要
                        'source': '国务院'
                    })
            
            logger.info(f"国务院抓取成功：{len(entries)} 条")
            
        except Exception as e:
            logger.error(f"国务院抓取失败：{e}")
        
        return entries
    
    def fetch_xinhua(self) -> List[Dict]:
        """抓取新华社时政新闻"""
        logger.info("抓取新华社时政新闻...")
        entries = []
        
        try:
            url = "https://www.xinhuanet.com/politics/"
            response = requests.get(url, headers=HEADERS, timeout=30)
            response.encoding = 'utf-8'
            soup = BeautifulSoup(response.text, 'lxml')
            
            # 查找新闻链接
            links = soup.select('a[href*="/politics/"]')
            
            seen = set()
            for link in links[:30]:
                title = link.get_text(strip=True)
                href = link.get('href', '')
                
                # 过滤重复和无效链接
                if not title or len(title) < 10 or href in seen:
                    continue
                if href.startswith('javascript:') or '#' in href:
                    continue
                    
                seen.add(href)
                full_url = href if href.startswith('http') else f"https://www.xinhuanet.com{href}"
                
                entries.append({
                    'title': title,
                    'link': full_url,
                    'published': datetime.now().isoformat(),
                    'summary': title[:200],
                    'source': '新华社'
                })
            
            logger.info(f"新华社抓取成功：{len(entries)} 条")
            
        except Exception as e:
            logger.error(f"新华社抓取失败：{e}")
        
        return entries
    
    def fetch_ndrc(self) -> List[Dict]:
        """抓取国家发改委"""
        logger.info("抓取国家发改委...")
        entries = []
        
        try:
            url = "https://www.ndrc.gov.cn/xwdt/xwfb/"
            response = requests.get(url, headers=HEADERS, timeout=30)
            response.encoding = 'utf-8'
            soup = BeautifulSoup(response.text, 'lxml')
            
            # 获取所有新闻链接
            all_links = soup.find_all('a', href=True)
            
            seen = set()
            for a in all_links:
                title = a.get_text(strip=True)
                href = a.get('href', '')
                
                # 过滤
                if not title or len(title) < 10:
                    continue
                if href in seen or href.startswith('javascript:') or '#' in href:
                    continue
                if './' in href or href.startswith('.'):
                    seen.add(href)
                    # 相对路径转绝对
                    base = url.rstrip('/').rsplit('/', 1)[0]
                    full_url = f"{base}/{href.lstrip('./')}"
                    
                    entries.append({
                        'title': title,
                        'link': full_url,
                        'published': datetime.now().isoformat(),
                        'summary': title[:200],
                        'source': '国家发改委'
                    })
                
                if len(entries) >= 20:
                    break
            
            logger.info(f"发改委抓取成功：{len(entries)} 条")
            
        except Exception as e:
            logger.error(f"发改委抓取失败：{e}")
        
        return entries
    
    def classify_policy(self, title: str, summary: str) -> str:
        """判定政策级别"""
        text = title + " " + summary
        
        # 检查一级关键词（立刻推送）
        level1_keywords = [
            "国务院令", "指导意见", "实施方案", "专项资金",
            "准入管理", "监管办法", "发展规划", "若干政策",
            "通知", "决定", "批复"
        ]
        
        for keyword in level1_keywords:
            if keyword in text:
                return "LEVEL1"
        
        # 检查二级关键词（周报汇总）
        level2_keywords = [
            "公告", "答复", "征求意见稿", "解读",
            "动态", "新闻", "会议"
        ]
        
        for keyword in level2_keywords:
            if keyword in text:
                return "LEVEL2"
        
        return "NORMAL"
    
    def save_policy(self, policy: Dict, level: str):
        """保存政策到数据库"""
        db_file = Path(__file__).parent / self.db_path
        conn = sqlite3.connect(str(db_file))
        cursor = conn.cursor()
        
        try:
            cursor.execute('''
                INSERT OR IGNORE INTO policies 
                (title, source, link, published, level, summary)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', (
                policy['title'],
                policy['source'],
                policy['link'],
                policy['published'],
                level,
                policy['summary']
            ))
            
            conn.commit()
            logger.info(f"保存政策：{policy['title'][:50]}...")
            
        except Exception as e:
            logger.error(f"保存失败：{e}")
        finally:
            conn.close()
    
    def get_unpushed_level1(self) -> List[Dict]:
        """获取未推送的一级政策"""
        db_file = Path(__file__).parent / self.db_path
        conn = sqlite3.connect(str(db_file))
        cursor = conn.cursor()
        
        cursor.execute('''
            SELECT id, title, source, summary, link
            FROM policies
            WHERE level = 'LEVEL1' AND pushed = 0
            ORDER BY created_at DESC
            LIMIT 5
        ''')
        
        rows = cursor.fetchall()
        conn.close()
        
        return [
            {
                'id': row[0],
                'title': row[1],
                'source': row[2],
                'summary': row[3],
                'link': row[4]
            }
            for row in rows
        ]
    
    def mark_pushed(self, policy_id: int):
        """标记政策已推送"""
        db_file = Path(__file__).parent / self.db_path
        conn = sqlite3.connect(str(db_file))
        cursor = conn.cursor()
        
        cursor.execute('''
            UPDATE policies SET pushed = 1 WHERE id = ?
        ''', (policy_id,))
        
        conn.commit()
        conn.close()
        logger.info(f"标记政策已推送：ID={policy_id}")
    
    def format_push_message(self, policy: Dict) -> str:
        """生成推送消息"""
        # 简化摘要（取前 100 字）
        summary = policy['summary'][:100] + "..." if len(policy['summary']) > 100 else policy['summary']
        
        # 影响范围（根据来源推断）
        impact = "全国性" if "国务院" in policy['source'] else "行业/地区"
        
        message = f"""🚨【政策速报】{policy['title']}
发布机构：{policy['source']}
核心内容：{summary}
影响范围：{impact}
原文链接：{policy['link']}"""
        
        return message
    
    def run(self):
        """运行抓取任务"""
        logger.info("=" * 50)
        logger.info("政策监控系统 V2 (HTML 版) 启动")
        logger.info("=" * 50)
        
        # 抓取各政府网站
        all_entries = []
        
        entries = self.fetch_gov_cn()
        all_entries.extend(entries)
        
        entries = self.fetch_xinhua()
        all_entries.extend(entries)
        
        entries = self.fetch_ndrc()
        all_entries.extend(entries)
        
        logger.info(f"总计抓取：{len(all_entries)} 条")
        
        # 分类并保存
        for entry in all_entries:
            level = self.classify_policy(entry['title'], entry['summary'])
            self.save_policy(entry, level)
        
        # 检查是否有需要推送的一级政策
        unpushed = self.get_unpushed_level1()
        
        if unpushed:
            logger.info(f"🔴 发现 {len(unpushed)} 条待推送政策！")
            
            for policy in unpushed:
                message = self.format_push_message(policy)
                logger.info(f"推送内容:\n{message}")
                
                # 标记已推送
                self.mark_pushed(policy['id'])
        else:
            logger.info("✅ 无待推送政策")
        
        logger.info("=" * 50)
        logger.info("抓取任务完成")
        logger.info("=" * 50)
        
        return len(all_entries), len(unpushed)


if __name__ == "__main__":
    crawler = PolicyCrawlerV2()
    total, to_push = crawler.run()
    print(f"\n总计抓取：{total} 条")
    print(f"待推送：{to_push} 条")
