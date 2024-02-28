import api, errors

if __name__ == "__main__":
    import sys
    import json
    configuration_file = sys.argv[1]
    configuration = json.load(open(configuration_file, 'r'))
    try:
        print(f"INFO: try to login as {configuration['UserName']}")
        user_id = api.login(configuration["UserName"], configuration["password"], configuration["schoolID"])
        print(f"INFO: Login Successed.")
        classes = api.get_classes_id(user_id)
        class_id = classes[0]["id"]
        print(f"INFO: select class {classes[0]['name']} {class_id}")
        print("INFO: Get All Articales")
        articles = api.get_articles(user_id=user_id, class_id=class_id)
        print(f"INFO: The number of Articles is {len(articles)}.")
        i, j = (0 ,0)
        while j < 2 and i < len(articles):
            try: 
                api.submit_essay_test(user_id, articles[i], class_id, api.get_essay_answer(articles[i]))
            except Exception as err:
                print(f"Error: {err}", file=sys.stderr)
            else:
                print(f"INFO: Succeed submit essay \"{articles[i]['title']}\"")
                j += 1
            finally:
                i += 1

    except Exception as err:
        print(f"Error: {err}", file=sys.stderr)
        exit(1)