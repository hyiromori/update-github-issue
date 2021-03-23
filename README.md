# update-github-issue

GitHub Issue の情報を (ZenHubも含めて) 更新するDeno製のCLIツール。

## 準備

### 環境変数

必要な環境変数を設定しておいてください

- `GITHUB_ACCESS_TOKEN`
  - GitHub API にアクセスするためのトークン
  - https://github.com/settings/tokens
  - 必要な権限: `repo`
- `ZENHUB_ACCESS_TOKEN`
  - ZenHub API にアクセスするためのトークン
  - https://app.zenhub.com/dashboard/tokens

### ダウンロード

[Releases](https://github.com/hyiromori/update-github-issue/releases) から使用したいバージョンのバイナリをダウンロードします。
バイナリをパスが通ったディレクトリに移動し、実行権限を付与してください。

## 実行方法

```bash
$ update-github-issue \
    --labels "label1,label2" \
    --pipeline "ZenHub Pipeline Name" \
    --workspace "ZenHub Workspace ID" \
    --epic-url 'GitHub Issue URL' \
    'GitHub URL'...
```

### ZenHub Workspace ID の取得方法

https://github.com/ZenHubIO/API#notes-1 を参照してください。
ZenHub の URL 中にあります。

## 更新可能な項目

### GitHub

- ラベル

### ZenHub

- 紐づけるEpic
- パイプライン

## Development

以下リンクを参考に Deno をインストールしてください。

https://github.com/denoland/deno_install/blob/master/README.md

### ビルド

```bash
$ ./build.sh
```

### ローカルでの実行

```bash
$ deno run --allow-net --allow-env ./src/index.ts [options...]
```
