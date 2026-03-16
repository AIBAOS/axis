#!/usr/bin/env python3
"""
RSS 源测试脚本
翰林院编制 - 验证官方 RSS 源连通性
"""

import feedparser
import requests
from typing import List, Dict


# 待测试 RSS 源清单
RSS_SOURCES = [
    {
        "name": "新华社政治新闻",
        "url": "http://www.xinhuanet.com/politicsnews/rss_politics.xml",
        "priority": "P0"
    },
    {
        "name": "央视新闻",
        "url": "https://m.cctv.com/api/rss/news/",
        "priority": "P0"
    },
    {
        "name": "国务院政策文件库",
        "url": "https://www.gov.cn/zhengce/zuixin.htm",  # 需确认 RSS
        "priority": "P0"
    },
    {
        "name": "国家发改委",
        "url": "https://www.ndrc.gov.cn/xwdt/gxdt/rss.xml",
        "priority": "P1"
    },
    {
        "name": "工信部",
        "url": "https://www.miit.gov.cn/xwdt/gxdt/rss.xml",
        "priority": "P1"
    },
    {
        "name": "财政部",
        "url": "http://www.mof.gov.cn/zhengwuxinxi/caizhengxinwen/",  # 需确认 RSS
        "priority": "P1"
    },
    {
        "name": "科技部",
        "url": "https://www.most.gov.cn/kjbgz/",  # 需确认 RSS
        "priority": "P1"
    },
    {
        "name": "人民日报政治",
        "url": "http://politics.people.com.cn/rss/24hr.xml",
        "priority": "P1"
    },
    {
        "name": "半月谈",
        "url": "http://www.banyuetan.org/ch/content/2022-09/23/content_100023412.htm",  # 需确认 RSS
        "priority": "P2"
    },
    {
        "name": "中国经济网",
        "url": "http://www.ce.cn/xwzx/gnsz/gdxw/",  # 需确认 RSS
        "priority": "P2"
    }
]


def test_rss(source: Dict) -> Dict:
    """测试单个 RSS 源"""
    result = {
        "name": source["name"],
        "url": source["url"],
        "priority": source["priority"],
        "status": "未知",
        "entries": 0,
        "note": ""
    }
    
    try:
        # 尝试解析 RSS
        feed = feedparser.parse(source["url"])
        
        if feed.bozo:
            result["status"] = "⚠️ 警告"
            result["note"] = f"解析警告：{feed.bozo_exception}"
        else:
            result["status"] = "✅ 可用"
            result["entries"] = len(feed.entries)
            
            if len(feed.entries) == 0:
                result["note"] = "RSS 正常但无条目"
            else:
                result["note"] = f"最新：{feed.entries[0].get('title', '无标题')[:50]}"
        
    except Exception as e:
        result["status"] = "❌ 失败"
        result["note"] = str(e)
    
    return result


def main():
    """主函数"""
    print("=" * 70)
    print("RSS 源连通性测试 - 翰林院")
    print("=" * 70)
    print()
    
    results = []
    
    for source in RSS_SOURCES:
        print(f"测试中：{source['name']}...")
        result = test_rss(source)
        results.append(result)
        
        status_icon = result["status"].split()[0]
        print(f"  {status_icon} {result['note'][:60]}")
    
    print()
    print("=" * 70)
    print("测试结果汇总")
    print("=" * 70)
    print()
    
    # 按优先级排序
    results.sort(key=lambda x: x["priority"])
    
    for r in results:
        print(f"{r['priority']} | {r['name']:<20} | {r['status']:<10} | {r['note'][:40]}")
    
    print()
    print("=" * 70)
    
    # 统计
    available = sum(1 for r in results if "✅" in r["status"])
    warning = sum(1 for r in results if "⚠️" in r["status"])
    failed = sum(1 for r in results if "❌" in r["status"])
    
    print(f"可用：{available} | 警告：{warning} | 失败：{failed}")
    print("=" * 70)


if __name__ == "__main__":
    main()
