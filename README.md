## Build
```bash
cargo build --release
```

## Usage
```bash
Usage: fuck-sibo.exe [COMMAND]

Commands:
  search  按学校名称查询Id
  fuck    自动完成文章练习
  ci      按配置文件设定自动完成文章练习
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

search:
按学校名称查询Id

Usage: fuck-sibo.exe search [name]

Arguments:
  [name]  

Options:
  -h, --help  Print help


ci:
Usage: fuck-sibo.exe ci [OPTIONS]

Options:
  -c <config>      
  -h, --help       Print help


fuck:
自动完成文章练习

Usage: fuck-sibo.exe fuck [OPTIONS]

Options:
  -t <thread>             线程数 [default: 1]
  -u <username>           
  -p <password>           
  -s <schoolId>           可通过search命令查找对应学校Id
  -n <articleNumber>      需完成篇数 [default: 1]
  -h, --help              Print help
```


## CI 配置文件格式
```toml
[config]
thread = 4
default_school_id = "example"
default_number_of_article = 2


[[user]]
username = "example1"
password = "pa$$w0rd"
school_id = "example_school" # 默认为 default_school_id, 可省略

[[user]]
username = "example2"
password = "pa$$w0rd"
number_of_article = 4 # 默认为 default_number_of_article, 可省略

[[user]]
username = "example3"
password = "pa$$w0rd"

[[user]]
username = "example4"
password = "pa$$w0rd"
school_id = "example_school"
number_of_article = 4 
```
