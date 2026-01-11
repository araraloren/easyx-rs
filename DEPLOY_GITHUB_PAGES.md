# 配置GitHub Pages

要启用GitHub Pages并访问自动部署的文档，请按照以下步骤操作：

## 1. 在GitHub仓库中启用GitHub Pages

1. 登录GitHub，进入你的仓库页面
2. 点击顶部导航栏的「Settings」（设置）按钮
3. 在左侧菜单中选择「Pages」
4. 在「Source」（源）部分，选择：
   - 分支：`gh-pages`
   - 目录：`/ (root)`
5. 点击「Save」（保存）按钮
6. 等待几秒钟，GitHub会显示你的GitHub Pages URL，例如：`https://<username>.github.io/easyx-rs/`

## 2. 触发文档部署工作流

1. 将所有代码推送到GitHub仓库的`main`分支
2. 在GitHub仓库页面，点击顶部导航栏的「Actions」
3. 你会看到「Deploy Documentation to GitHub Pages」工作流正在运行
4. 等待工作流完成（通常需要1-2分钟）
5. 工作流完成后，访问你的GitHub Pages URL即可查看文档

## 3. 手动触发工作流（可选）

如果需要手动触发文档部署：

1. 进入GitHub仓库的「Actions」页面
2. 点击左侧的「Deploy Documentation to GitHub Pages」工作流
3. 点击「Run workflow」按钮
4. 选择分支（默认为`main`），点击「Run workflow」

## 4. 验证部署

1. 访问你的GitHub Pages URL：`https://<username>.github.io/easyx-rs/`
2. 你应该会看到自动重定向到`easyx/index.html`，显示完整的文档
3. 可以通过URL直接访问特定模块，例如：`https://<username>.github.io/easyx-rs/easyx_sys/`

## 5. 常见问题排查

### 问题：GitHub Pages显示404错误

**解决方法**：
1. 检查工作流是否成功运行：在「Actions」页面查看工作流状态
2. 检查`gh-pages`分支是否存在：在「Code」页面切换分支查看
3. 等待10-15分钟，GitHub Pages可能需要时间传播
4. 检查GitHub Pages设置是否正确：确认分支和目录选择正确

### 问题：工作流运行失败

**解决方法**：
1. 点击失败的工作流运行，查看详细日志
2. 常见错误包括：
   - 权限问题：确保工作流有写入GitHub Pages的权限
   - 构建错误：检查Cargo.toml和代码是否有语法错误
   - 路径问题：确保文档生成到正确的目录

## 6. 本地文档生成

你也可以在本地生成和查看文档：

```bash
# 生成文档（不包含依赖）
cargo doc --no-deps

# 生成并自动打开文档
cargo doc --no-deps --open
```

生成的文档位于`target/doc/`目录中，可以用浏览器打开`target/doc/easyx/index.html`查看。