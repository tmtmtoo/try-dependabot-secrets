# try-dependabot-secrets

- dependabot コンテキストで実行される workflow は actions secret を読めない
- ある workflow を dependabot が作成した PR イベントでも実行可能にするために dependabot secret を設定する必要がある
- dependabot が使用する secret は actions secret と同じ名前にする
- dependabot が作成する PR に auto merge workflow を仕掛ける場合は merge 後に push event が発火しない
  - https://umihi.co/blog/20221019-run-gh-as-actual-user-instead-of-github-actions-bot-in-actions
- push event を発火させるために merge 実行時の GITHUB_TOKEN を差し替えた場合はトークンの持ち主のユーザーとして merge する
- push evnet により実行される workflow はユーザーコンテキストで実行されるので actions secret を読むことができる
