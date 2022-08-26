# intellidyn_cloud
An Enterprise Resource Planning Software built in Rust and AWS

# APIs

> CreateTable
    https://7s4exxozcg.execute-api.us-east-1.amazonaws.com/dev/create_table?name=company&key=company_key
    https://7s4exxozcg.execute-api.us-east-1.amazonaws.com/dev/create_table?name=company_code&key=company_code

-PPIE
--Instance IMG
---Enterprise Structure
----Definition
-----Financial Accounting
------Define Company
            https://6cdw15j8z9.execute-api.us-east-1.amazonaws.com/dev/define_company

------Edit, Copy, Delete, Check Company Code
            https://f43b1z4pb9.execute-api.us-east-1.amazonaws.com/dev/define_company_code

> List Companies
    https://ludlgaugei.execute-api.us-east-1.amazonaws.com/dev/list_companies

> List CompanyCodes
    https://grvhs1pamd.execute-api.us-east-1.amazonaws.com/dev/list_company_codes