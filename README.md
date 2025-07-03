## Rust Web 后端开发学习

### 技术栈
- 后端框架 [axum](https://crates.io/crates/axum)
- 日志框架 [tracing](https://crates.io/crates/tracing)
- ORM 框架 [sea-orm](https://crates.io/crates/sea-orm)
- 数据库支持 [PostgreSQL](https://www.postgresql.org)

### 目前实现的功能
- 日志集成
- 配置文件
- 数据校验的自定义
- 数据库连接，以及查询
- 完成所有的 CRUD 功能，包括联表查询
- 部分支持 IPv6（IPv4 和 IPv6 只能开启一个，无法支持双栈）
- 用户登陆
- 用户登录加密

### 目标
- 完全支持 IPv6

### 功能详细说明
#### 日志集成
可通过 `web-start.toml` 配置输出日志的等级
#### 配置文件
创建一个 `web-start.toml` 文件以实现配置功能
```toml
# 目前支持的所有字段
[server]                # 控制服务器属性
port = 8080             # 端口号——默认值 8080
log_level = "info"      # 默认日志输出等级为 info 及以上的日志
ipv4_enabled = true     # 开启 IPv4 ( 两个只能开启一个 )
ipv6_enalbed = false    # 开启 IPv6 ( 两个只能开启一个 )
# 默认值为这一长串，是 "default secret key of web-starter" 的 base64 编码，这个字段必须是一个合法的可用 base 64 解码的字符串
secret_key = "ZGVmYXVsdCBzZWNyZXQga2V5IG9mIHdlYi1zdGFydGVy"

# 仅支持 PostgreSQL
[database]              # 控制数据库连接
host = "127.0.0.1"      # 运行数据库实例的主机的 IP 地址，默认本机
port = 5432             # 数据库实例在该主机上守候的端口号，默认 5432
user = "postgres"       # 登录数据库的用户名，默认 postgres
passwd = "123456"       # 此用户名的密码，无默认值，必须填写
database = "postgres"   # 要连接的数据库，默认 postgres
schema = "public"       # 要连接的数据库的模式，默认 public
```
#### 数据校验的自定义
通过桥接 `axum-valid` 的 `valid`，实现了校验错误的统一
#### 数据库连接
目前仅支持 PostgreSQL，未来大概率也不会支持其他 DBMS，因为这只是一个学习项目
#### 增删查改
通过连接查询支持了以下功能：
- 查询某一个学院的所有学生
- 查询某一个学院开设的所有课程
- 查询某一个学生在某一个课程的的成绩
- 查询某一个学生的所有成绩
- 查询某一个课程的所有成绩
#### 登录功能
通过 login 页面生成一个 JWT 返回给浏览器，浏览器通过携带这个 JWT 访问受保护的页面，目前受保护的页面为除了 `login` 页面之外的所有页面