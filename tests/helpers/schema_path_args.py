from os import path


def schema_path_args(file: str, schema_file_name: str) -> list[str]:
    schema_file_path = path.join(path.dirname(file), schema_file_name)
    return ["python", "teo", "serve", "--schema", schema_file_path]
