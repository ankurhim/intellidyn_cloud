build-IntellidynCloudFunction:
	@echo "compiling...."
	cargo build --release --target x86_64-unknown-linux-musl

	# @echo "creating create_table bootstrap zip file..."
	# cp ./target/x86_64-unknown-linux-musl/release/create_table ./bootstrap
	# zip -r bootstrap.zip bootstrap

	# @echo "uploading to s3 bucket..."
	# aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	# @echo "create lambda function..."
	# aws lambda create-function \
	# --function-name CreateTable \
	# --runtime provided.al2 \
	# --handler main.create_table \
	# --role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	# --zip-file fileb://bootstrap.zip

	@echo "creating define_company bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/define_company ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name DefineCompany \
	--runtime provided.al2 \
	--handler main.define_company \
	--role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	--zip-file fileb://bootstrap.zip

	@echo "creating list_companies bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/list_companies ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name ListCompanies \
	--runtime provided.al2 \
	--handler main.list_companies \
	--role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	--zip-file fileb://bootstrap.zip

	@echo "creating define_company_code bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/define_company_code ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name DefineCompanyCode \
	--runtime provided.al2 \
	--handler main.define_company_code \
	--role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	--zip-file fileb://bootstrap.zip