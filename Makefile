build-IntellidynCloudFunction:
	@echo "compiling...."
	cargo build --release --target x86_64-unknown-linux-musl

	@echo "creating list_company_codes bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/list_company_codes ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name ListCompanyCodes \
	--runtime provided.al2 \
	--handler main.list_company_codes \
	--role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	--zip-file fileb://bootstrap.zip