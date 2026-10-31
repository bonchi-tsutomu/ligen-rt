# LigenRT
リンク集をWeb上で編集するためのサーバーソフトウェア

## 設定ファイル
設定ファイルはYAMLで記述してください。以下はファイルの例です。

```yaml
site:
  title: "My Links" #サイト名です
  description: "Lorem Ipsum Dolor Sit Amet" # サイトの概要
  guidelines: ["Hello", "World", "Foo", "Bar"] # 短いガイドラインを記述してください
  allow_request: true
server:
  bind: localhost
  port: 8080
  theme: ./theme
database: # SQLite3のみ
  dbname: test.db
```

## 最初に実行するSQL
```sql
CREATE TABLE link (
    id VARCHAR(8) UNIQUE NOT NULL PRIMARY KEY,
    url TEXT NOT NULL,
    contributor TEXT NOT NULL,
    description TEXT NOT NULL,
    counter INTEGER NOT NULL
);

CREATE TABLE links (
    id VARCHAR(8) UNIQUE NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL
);

CREATE TABLE link_attribute (
    linkid VARCHAR(8) NOT NULL,
    linksid VARCHAR(8) NOT NULL
);

CREATE TABLE category (
    id VARCHAR(8) NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);
```