from . import api
from . import errors

if __name__ == "__main__":
    import sys
    import json
    configuration_file = sys.argv[1]
    configuration = json.load(open(configuration_file, 'r'))
    try:
        user_id = api.login(configuration["UserName"], configuration["password"], configuration["schoolID"])
        classes = api.get_classes_id(user_id)
        print("INFO: ")
        class_id = classes[0]["id"]
    except Exception as err:
        print(f"Error: {err}", file=sys.stderr)
        exit(1)