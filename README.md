**随机生成学习任务**😅

```bash
- 【日常】: 基础知识 标准库 > TODO > 生态内容
- 【日常】: 笔记复习、迁移、整理
- 【实战任务】: 进行实战任务
- 【日常】: 整理当天的技术文章并记录到对应todo * 1
- 【支线】: 英语(单词20 或 书籍一小节)
```

<br>

使用: 直接使用dist目录下的预构建版本, 目前仅预构建了 aarch64 的 macos 和 x86_64 的 windows版本

自行编译: 安装 rust 后进行编译

1. 克隆并进入项目目录, 运行 `cargo build --release`
2. 复制可执行文件 `target/release/task-generate` 到任意目录
3. 在执行文件所在目录创建`task.json`, 录入任务信息:
   - label 任务名
   - probability 任务出现概率(0-1)
   - restDayProbability 休息日出现概率(0-1)
   - randCategory 随机选项, 会从这些选项中根据 ratio 比例随机选取并根据顺序设置到 label 的模板变量`$category<1...n>`中
4. 运行可执行文件, 输出生成的任务

<br>

一个任务配置示例:

```json
{
  "restDayProbability": 0.03,
  "tasks": [
    {
      "label": "【日常】: $category1 标准库 > TODO > 生态内容 ",
      "randCategory": [
        [
          {
            "label": "前端",
            "ratio": 0.4
          },
          {
            "label": "后端",
            "ratio": 0.4
          },
          {
            "label": "基础知识",
            "ratio": 0.2
          }
        ]
      ],
      "probability": 0.95
    },
    {
      "label": "【日常】: 阅读",
      "probability": 0.7
    },
    {
      "label": "【日常】: 笔记复习、迁移、整理",
      "probability": 0.5
    },
    {
      "label": "【实战任务】: 进行实战任务",
      "probability": 0.9
    },
    {
      "label": "【日常】: 整理当天的技术文章并记录到对应todo * 1"
    },
    {
      "label": "【支线】: 英语(单词20 或 书籍一小节)"
    }
  ]
}
```
