在 Linux 服务器上启动 Docusaurus 会遇到一个错误:

```bash
Error: ENOSPC: System limit for number of file watchers reached, watch '/home/xfy/Git/rust-playground/doc/sidebars.js'
```

最近也没有修改配置文件，突然就遇到这个错误。根据错误提示，很明显是和系统本身相关的。简单的了解了下，还是因为 Linux watches 数量的问题。

最简单的解决办法还是修改位于 `/etc/sysctl.conf` 的这个文件，添加一行：

```
fs.inotify.max_user_watches=524288
```

并使其立即生效：`sysctl -p`