#!/usr/bin/env python3
"""
政策监控系统 - 抓取脚本 V1
翰林院编制 - 中国政府政策监控

功能：
1. 抓取 RSS 源
2. 识别重大政策
3. 生成推送内容
4. 记录到 SQLite 数据库
"""

import feedparser
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


class PolicyCrawler:
    """政策抓取器"""
    
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
    
    def fetch_rss(self, url: str) -> List[Dict]:
        """抓取 RSS 源"""
        logger.info(f"抓取 RSS: {url}")
        
        try:
            feed = feedparser.parse(url)
            
            if feed.bozo:
                logger.warning(f"RSS 解析警告：{feed.bozo_exception}")
            
            entries = []
            for entry in feed.entries:
                entries.append({
                    'title': entry.get('title', ''),
                    'link': entry.get('link', ''),
                    'published': entry.get('published', ''),
                    'summary': entry.get('summary', '')[:500] if entry.get('summary') else '',
                    'source': feed.feed.get('title', '未知来源')
                })
            
            logger.info(f"抓取成功：{len(entries)} 条")
            return entries
            
        except Exception as e:
            logger.error(f"抓取失败：{e}")
            return []
    
    def classify_policy(self, title: str, summary: str) -> str:
        """判定政策级别"""
        text = title + " " + summary
        
        # 检查一级关键词
        for keyword in self.config['policy_keywords']['level1']:
            if keyword in text:
                return "LEVEL1"  # 立刻推送
        
        # 检查二级关键词
        for keyword in self.config['policy_keywords']['level2']:
            if keyword in text:
                return "LEVEL2"  # 周报汇总
        
        return "NORMAL"  # 普通新闻
    
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
        template = self.config['push']['template']
        
        # 简化摘要（取前 100 字）
        summary = policy['summary'][:100] + "..." if len(policy['summary']) > 100 else policy['summary']
        
        # 影响范围（根据来源推断）
        impact = "全国性" if "国务院" in policy['source'] or "新华社" in policy['source'] else "行业/地区"
        
        message = template.format(
            title=policy['title'],
            source=policy['source'],
            summary=summary,
            impact=impact
        )
        
        return message
    
    def run(self):
        """运行抓取任务"""
        logger.info("=" * 50)
        logger.info("政策监控系统启动")
        logger.info("=" * 50)
        
        # 抓取所有启用的 RSS 源
        for source in self.config['rss_sources']:
            if not source.get('enabled', False):
                continue
            
            entries = self.fetch_rss(source['url'])
            
            for entry in entries:
                level = self.classify_policy(entry['title'], entry['summary'])
                self.save_policy(entry, level)
        
        # 检查是否有需要推送的一级政策
        unpushed = self.get_unpushed_level1()
        
        if unpushed:
            logger.info(f"发现 {len(unpushed)} 条待推送政策")
            
            for policy in unpushed:
                message = self.format_push_message(policy)
                logger.info(f"推送内容:\n{message}")
                
                # 标记已推送
                self.mark_pushed(policy['id'])
        else:
            logger.info("无待推送政策")
        
        logger.info("=" * 50)
        logger.info("抓取任务完成")
        logger.info("=" * 50)


if __name__ == "__main__":
    crawler = PolicyCrawler()
    crawler.run()
