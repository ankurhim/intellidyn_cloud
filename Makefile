build-IntellidynCloudFunction:
	@echo "compiling...."
	cargo build --release --target x86_64-unknown-linux-musl
	@echo "creating intellidyn_cloud..."
	cp ./target/x86_64-unknown-linux-musl/release/connection ./connection
	zip -r connection.zip connection

	@echo "uploading to s3 bucket..."
	aws s3 cp connection.zip s3://intellidyn-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name Connection \
	--runtime provided.al2 \
	--handler main.connect \
	--role arn:aws:iam::105390037103:role/intellidyn-role \
	--zip-file fileb://connection.zip
