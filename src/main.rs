use clap::{Arg, Command};
use colored::*;
use inquire::{Select, Text, Confirm};
use std::process;
use regex::Regex;

// Angular commit 类型
const COMMIT_TYPES: &[(&str, &str)] = &[
    ("feat", "新功能 (A new feature)"),
    ("fix", "修复bug (A bug fix)"),
    ("docs", "文档更新 (Documentation only changes)"),
    ("style", "代码格式 (Changes that do not affect the meaning of the code)"),
    ("refactor", "重构 (A code change that neither fixes a bug nor adds a feature)"),
    ("perf", "性能优化 (A code change that improves performance)"),
    ("test", "测试 (Adding missing tests or correcting existing tests)"),
    ("build", "构建系统 (Changes that affect the build system or external dependencies)"),
    ("ci", "CI配置 (Changes to our CI configuration files and scripts)"),
    ("chore", "其他杂务 (Other changes that don't modify src or test files)"),
    ("revert", "回滚 (Reverts a previous commit)"),
];

fn main() {
    let app = Command::new("commit-formatter")
        .version("v1.0.0-alpha")
        .author("Yule")
        .about("生成符合Angular规范的Git commit消息")
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("交互式模式")
                .action(clap::ArgAction::SetTrue),
        );

    let matches = app.get_matches();

    if matches.get_flag("interactive") || std::env::args().len() == 1 {
        run_interactive_mode();
    } else {
        println!("{}", "使用 --interactive 或 -i 启动交互式模式".yellow());
    }
}

fn run_interactive_mode() {
    println!("{}", "Commit消息生成器".bright_blue().bold());
    println!("{}", "让我们一步步创建符合Angular规范的commit消息\n".cyan());

    // 1. 选择commit类型
    let commit_type = select_commit_type();
    
    // 2. 输入作用域 (可选)
    let scope = input_scope();
    
    // 3. 输入简短描述
    let description = input_description();
    
    // 4. 输入详细描述 (可选)
    let body = input_body();
    
    // 5. 输入破坏性变更 (可选)
    let breaking_change = input_breaking_change();
    
    // 6. 输入关联的issue (可选)
    let issues = input_issues();
    
    // 生成commit消息
    let commit_message = generate_commit_message(
        &commit_type,
        &scope,
        &description,
        &body,
        &breaking_change,
        &issues,
    );
    
    // 显示结果
    display_result(&commit_message);
}

fn select_commit_type() -> String {
    let options: Vec<String> = COMMIT_TYPES
        .iter()
        .map(|(t, desc)| format!("{}: {}", t.bright_green(), desc))
        .collect();
    
    let selection = Select::new("选择commit类型:", options)
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "操作已取消".red());
            process::exit(1);
        });
    
    // 提取类型名称
    selection.split(':').next().unwrap().trim().to_string()
}

fn input_scope() -> Option<String> {
    let scope = Text::new("输入作用域 (可选，如: auth, ui, api):")
        .with_help_message("作用域表示此次更改影响的模块或组件")
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "操作已取消".red());
            process::exit(1);
        });
    
    if scope.trim().is_empty() {
        None
    } else {
        Some(scope.trim().to_string())
    }
}

fn input_description() -> String {
    Text::new("输入简短描述 (必填):")
        .with_help_message("用现在时态描述此次更改，不超过50个字符")
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(inquire::validator::Validation::Invalid(
                    "描述不能为空".into(),
                ))
            } else if input.len() > 50 {
                Ok(inquire::validator::Validation::Invalid(
                    "描述不应超过50个字符".into(),
                ))
            } else {
                Ok(inquire::validator::Validation::Valid)
            }
        })
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "操作已取消".red());
            process::exit(1);
        })
}

fn input_body() -> Option<String> {
    let add_body = Confirm::new("是否添加详细描述?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    
    if add_body {
        println!("{}", "请输入详细描述 (支持多行，输入空行结束):".cyan());
        println!("{}", "提示: 详细解释此次更改的动机和实现方式".dimmed());
        
        let mut lines = Vec::new();
        loop {
            let line = Text::new(&format!("第{}行:", lines.len() + 1))
                .with_default("")
                .prompt()
                .unwrap_or_else(|_| {
                    println!("{}", "操作已取消".red());
                    process::exit(1);
                });
            
            if line.trim().is_empty() {
                break;
            }
            
            lines.push(line);
        }
        
        if lines.is_empty() {
            None
        } else {
            Some(lines.join("\n"))
        }
    } else {
        None
    }
}

fn input_breaking_change() -> Option<String> {
    let has_breaking = Confirm::new("是否包含破坏性变更?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    
    if has_breaking {
        let breaking = Text::new("描述破坏性变更:")
            .with_help_message("描述不兼容的API变更")
            .prompt()
            .unwrap_or_else(|_| {
                println!("{}", "操作已取消".red());
                process::exit(1);
            });
        
        if breaking.trim().is_empty() {
            None
        } else {
            Some(breaking.trim().to_string())
        }
    } else {
        None
    }
}

fn input_issues() -> Option<String> {
    let add_issues = Confirm::new("是否关联issue?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    
    if add_issues {
        let issues = Text::new("输入issue编号 (如: #123, #456):")
            .with_help_message("多个issue用逗号分隔")
            .prompt()
            .unwrap_or_else(|_| {
                println!("{}", "操作已取消".red());
                process::exit(1);
            });
        
        if issues.trim().is_empty() {
            None
        } else {
            Some(issues.trim().to_string())
        }
    } else {
        None
    }
}

fn generate_commit_message(
    commit_type: &str,
    scope: &Option<String>,
    description: &str,
    body: &Option<String>,
    breaking_change: &Option<String>,
    issues: &Option<String>,
) -> String {
    let mut message = String::new();
    
    // 构建标题行
    let mut title = commit_type.to_string();
    if let Some(s) = scope {
        title.push_str(&format!("({})", s));
    }
    
    // 如果有破坏性变更，添加感叹号
    if breaking_change.is_some() {
        title.push('!');
    }
    
    title.push_str(": ");
    title.push_str(description);
    message.push_str(&title);
    
    // 添加详细描述
    if let Some(b) = body {
        message.push_str("\n\n");
        message.push_str(b);
    }
    
    // 添加破坏性变更
    if let Some(bc) = breaking_change {
        message.push_str("\n\nBREAKING CHANGE: ");
        message.push_str(bc);
    }
    
    // 添加关联的issues
    if let Some(i) = issues {
        message.push_str("\n\n");
        // 处理多个issue
        let issue_list: Vec<&str> = i.split(',').map(|s| s.trim()).collect();
        for issue in issue_list {
            if issue.starts_with('#') {
                message.push_str(&format!("Closes {}\n", issue));
            } else {
                message.push_str(&format!("Closes #{}\n", issue));
            }
        }
        message.pop(); // 移除最后的换行符
    }
    
    message
}

// 移除字符串中的ANSI颜色转义码
fn remove_ansi_colors(s: &str) -> String {
    let re = Regex::new("\x1b\\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

fn display_result(commit_message: &str) {
    println!("\n{}", "=== 生成的Commit消息 ===".bright_yellow().bold());
    println!("{}", commit_message.bright_white());

    println!("\n{}", "=== Git命令 ===".bright_yellow().bold());
    // 移除颜色转义符，用于生成 git 命令
    let clean_commit_message = remove_ansi_colors(commit_message);
    let git_command = generate_git_command(&clean_commit_message);
    println!("{}", git_command.bright_green());
    println!("\n{}", "复制上面的git命令并在终端中执行即可提交！".cyan());

    // 询问是否直接执行commit
    let execute = Confirm::new("是否直接执行git commit命令?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    if execute {
        // 传递清除过颜色转义符的提交信息
        execute_git_commit(&clean_commit_message);
    }
}

fn generate_git_command(commit_message: &str) -> String {
    let lines: Vec<&str> = commit_message.lines().collect();
    
    if lines.len() <= 1 {
        // 单行消息，使用单个-m
        format!("git commit -m \"{}\"", commit_message.replace('"', "\\\""))
    } else {
        // 多行消息，每行使用一个-m参数
        let mut command = String::from("git commit");
        let mut is_first = true;
        
        for line in lines {
            if !line.trim().is_empty() {
                command.push_str(&format!(" -m \"{}\"", line.replace('"', "\\\"")))
            } else if !is_first {
                // 空行用于分隔不同部分，但跳过开头的空行
                command.push_str(" -m \"\"");
            }
            is_first = false;
        }
        command
    }
}

fn execute_git_commit(message: &str) {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("{}", "Commit成功！".bright_green().bold());
                if !result.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&result.stdout));
                }
            } else {
                println!("{}", "Commit失败！".bright_red().bold());
                if !result.stderr.is_empty() {
                    println!("{}", String::from_utf8_lossy(&result.stderr).red());
                }
            }
        }
        Err(e) => {
            println!("{}", format!("执行git命令时出错: {}", e).bright_red());
        }
    }
}
