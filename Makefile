build-IntellidynCloudFunction:
	@echo "compiling...."
	cargo build --release --target x86_64-unknown-linux-musl

	@echo "creating assign_company_code_to_company bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/assign_company_code_to_company ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name AssignCompanyCodeToCompany2 \
	--runtime provided.al2 \
	--handler main.assign_company_code_to_company \
	--role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	--zip-file fileb://bootstrap.zip