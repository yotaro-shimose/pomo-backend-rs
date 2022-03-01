from dotenv import load_dotenv
from pathlib import Path
import os
import argparse
import subprocess
from argparse import Namespace

DOTENV_PATH = Path().joinpath(".env")
load_dotenv(DOTENV_PATH)
BIN_PATH = Path().joinpath("runner", "src", "lambda")
AWS_EXECUTION_ROLE = os.environ.get("AWS_EXECUTION_ROLE")
AWS_ACCOUNT_ID = os.environ.get("AWS_ACCOUNT_ID")
CLIENT_ID = os.environ.get("CLIENT_ID")
CLIENT_SECRET = os.environ.get("CLIENT_SECRET")
TOKEN_URI = os.environ.get("TOKEN_URI")
AUTH_URI = os.environ.get("AUTH_URI")


def build_lambda_function(function_name: str):
    subprocess.run(
        f"cross build --bin {function_name} --release --target x86_64-unknown-linux-musl",
        shell=True,
    )


def prepare_zip(function_name: str):
    build_lambda_function(function_name)
    subprocess.run(
        f"cp ./target/x86_64-unknown-linux-musl/release/{function_name} ./bootstrap",
        shell=True,
    )
    subprocess.run(f"zip {function_name}.zip bootstrap", shell=True)
    subprocess.run(f"rm bootstrap", shell=True)


def create_lambda_function(function_name: str):
    prepare_zip(function_name)
    subprocess.run(
        f"""
aws lambda create-function --function-name {function_name} \
--handler doesnt.matter \
--zip-file fileb://./{function_name}.zip \
--runtime provided.al2 \
--role arn:aws:iam::{AWS_ACCOUNT_ID}:role/service-role/{AWS_EXECUTION_ROLE} \
--environment Variables={{RUST_BACKTRACE=1,CLIENT_ID={CLIENT_ID},CLIENT_SECRET={CLIENT_SECRET},TOKEN_URI={TOKEN_URI},AUTH_URI={AUTH_URI}}} \
--tracing-config Mode=Active
""",
        shell=True,
    )
    clean_zip(function_name)


def update_lambda_function(function_name: str):
    prepare_zip(function_name)
    subprocess.run(
        f"""\
aws lambda update-function-code \
--function-name {function_name} \
--zip-file fileb://./{function_name}.zip \
""",
        shell=True,
    )
    clean_zip(function_name)


def delete_lambda_function(function_name: str):
    subprocess.run(
        f"aws lambda delete-function --function-name {function_name}", shell=True
    )


def clean_zip(function_name: str):
    Path().joinpath(f"{function_name}.zip").unlink(missing_ok=False)


def create_handler(args: Namespace):
    binaries: list[str] = args.binaries
    if len(binaries) == 0:
        binaries = [binary.stem for binary in BIN_PATH.iterdir()]
    for function_name in binaries:
        create_lambda_function(function_name)


def update_handler(args: Namespace):
    binaries: list[str] = args.binaries
    if binaries is None:
        raise ValueError("You have to specify target binaries to delete")
    for function_name in binaries:
        update_lambda_function(function_name)


def delete_handler(args: Namespace):
    binaries: list[str] | None = args.binaries
    if binaries is None:
        raise ValueError("You have to specify target binaries to delete")
    for function_name in binaries:
        delete_lambda_function(function_name)


# Parser
parser = argparse.ArgumentParser(description="Lambda Deploy Script")
sub_parsers = parser.add_subparsers()

# Create
create_parsers = sub_parsers.add_parser("create", help="create new lambda function")
create_parsers.add_argument(
    "-b",
    "--binaries",
    type=str,
    help="target binary names to deploy",
    nargs="*",
    default=list(),
)
create_parsers.set_defaults(handler=create_handler)

# Update
update_parsers = sub_parsers.add_parser(
    "update", help="update existing lambda function"
)
update_parsers.add_argument(
    "-b",
    "--binaries",
    type=str,
    help="target binary name to deploy",
    nargs="*",
)
update_parsers.set_defaults(handler=update_handler)

# Delete
delete_parsers = sub_parsers.add_parser(
    "delete", help="delete existing lambda function"
)
delete_parsers.add_argument(
    "-b",
    "--binaries",
    type=str,
    help="target binary name to deploy",
    nargs="*",
)
delete_parsers.set_defaults(handler=delete_handler)

if __name__ == "__main__":
    args = parser.parse_args()
    if hasattr(args, "handler"):
        args.handler(args)
    else:
        # 未知のサブコマンドの場合はヘルプを表示
        parser.print_help()
