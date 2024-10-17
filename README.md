
- https://qiita.com/Yuki_Oshima/items/860a859fb85365a609fc
- https://makky12.hatenablog.com/entry/2023/08/21/120500
- https://zenn.dev/mo_ri_regen/articles/aws-cli-setting
- https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/rust-package.html


```bash
# Create
aws lambda create-function \
  --function-name rust-test \
  --runtime provided.al2023 \
  --role arn:aws:iam::973776200248:role/iam-role-lambda-execute \
  --handler rust.handler \
  --zip-file fileb://target/lambda/event_consumer/bootstrap.zip
  
# Update
aws lambda update-function-code \
  --function-name rust-test \
  --zip-file fileb://target/lambda/event_consumer/bootstrap.zip
```
