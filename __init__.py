import datetime
from logging import LogRecord
import loguru
import api, errors

if __name__ == "__main__":
    import sys
    import json
    from loguru import logger
    import pytz

    def TimeZonePatcher(record):
        # record..astimezone(pytz.timezone("Asia/Shanghai"))
        record["time"] = datetime.datetime.now().astimezone(pytz.timezone("Asia/Shanghai"))

    logger.remove(0) # remove default handler
    logger.add(sys.stdout, format="[<green>{time:YYYY-MM-DD HH:mm:ss}</green>] [<level>{level}</level>] <level>{message}</level>")
    logger.add('log.log', format="[{time:YYYY-MM-DD HH:mm:ss}] [{level}] {message}")
    logger.patch(TimeZonePatcher)
    configuration_file = sys.argv[1]
    configuration = json.load(open(configuration_file, 'r'))
    try:
        logger.info(f"try to login as {configuration['UserName']}")
        user_id = api.login(configuration["UserName"], configuration["password"], configuration["schoolID"])
        logger.success("Login Successed.")
        classes = api.get_classes_id(user_id)
        class_id = classes[0]["id"]
        logger.info(f"select class {classes[0]['name']} {class_id}")
        logger.info("Get All Articales")
        articles = api.get_articles(user_id=user_id, class_id=class_id)
        logger.success(f"The number of Articles is {len(articles)}.")
        i, j = (0 ,0)
        while j < 2 and i < len(articles):
            try: 
                api.submit_essay_test(user_id, articles[i], class_id, api.get_essay_answer(articles[i]))
            except Exception as err:
                logger.error(str(err))
            else:
                logger.success(f"Succeed submit essay \"{articles[i]['title']}\"")
                j += 1
            finally:
                i += 1
    except Exception as err:
        logger.error(str(err), file=sys.stderr)
        exit(1)