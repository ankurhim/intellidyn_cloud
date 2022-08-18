build-IntellidynCloudFunction:
	@echo "compiling...."
	cargo build --release --target x86_64-unknown-linux-musl

	@echo "creating create_table bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/create_table ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name CreateTable \
	--runtime provided.al2 \
	--handler main.create_table \
	--role arn:aws:iam::105390037103:role/intellidyn-role \
	--zip-file fileb://bootstrap.zip \

	exit
