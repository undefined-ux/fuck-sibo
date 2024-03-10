
import api, errors
import loguru

__all__ = [
    "api", "errors", "start"
]

def __initalize_logger(log_path: str = 'log.log', logger = loguru.logger):
    logger.remove(0)
    logger.add(sys.stderr, format="[<green>{time:YYYY-MM-DD HH:mm:ss}</green>] [<level>{level}</level>] <level>{message}</level>")
    logger.add(log_path, format="[{time:YYYY-MM-DD HH:mm:ss}] [{level}] {message}")
    return logger

def __read_configuration_file(configuration_path: str, logger = loguru.logger) -> dict:
    try:
        return json.load(open(configuration_path, 'r'))
    except Exception as err:
        logger.critical(f"Failed to read configuration file {configuration_path}: {err}")
        exit(1)

def start(user_name: str, password: str, school_id: str, number: int = 2, logger = loguru.logger):
    try:
        logger.info(f"[{user_name}] try to login as {user_name}")
        user_info= api.login(user_name, password, school_id)
        user_id = user_info["user_id"]
        
        logger.success(f"[{user_name}] Login Successed.")
        classes = api.get_classes_id(user_id)
        class_id = classes[0]["id"]
        logger.info(f"[{user_name}] select class {classes[0]['name']} {class_id}")
        logger.info(f"[{user_name}] Get All Articales")
        articles = api.get_articles(user_id=user_id, class_id=class_id)
        logger.success(f"[{user_name}] The number of Articles is {len(articles)}.")
        i, j = (0 ,0)
        while j < number and i < len(articles):
            try: 
                api.read_essay(user_id, articles[i], class_id)
                api.submit_essay_test(user_id, articles[i], class_id, api.get_essay_answer(articles[i]))
            except (errors.SubmitEssayTestError, errors.ReadEssayError) as err:
                logger.warning(f"[{user_name}] {err}")
            except Exception as err:
                logger.error(f"[{user_name}] {err}")
            else:
                logger.success(f"[{user_name}] Succeed submit essay \"{articles[i]['title']}\"")
                j += 1
            finally:
                i += 1
    except Exception as err:
        logger.error(f"[{user_name}] {err}")
        exit(1)


if __name__ == "__main__":
    import sys
    import json
    import threading
    
    logger = __initalize_logger()
    if len(sys.argv) != 2:
        logger.critical("Invaild Argument.")
        exit(1)
        
    configuration_file_path = sys.argv[1]
    configuration = __read_configuration_file(configuration_file_path, logger)
    
    thread_num: int = configuration.get("threadNumber", 1)
    logger.info(f"Using {thread_num} threads")
    
    threads: list[threading.Thread] = []
    users: list[dict] = configuration["users"]
    while len(users):
        if len(threads) <= thread_num and len(users):
            user: dict[str, str] = users.pop()
            new_thread = threading.Thread(
                    target=lambda: start(
                        user_name=user["userName"],
                        password=user["password"],
                        school_id=user["schoolID"], 
                        number=int(user["LessonNumber"]),
                        logger=logger
                    )
                )
            new_thread.run()
            threads.append(new_thread)
        for thread in threads:
            if not thread.is_alive():
                threads.remove(thread)
                if not len(users): continue
                user: dict[str, str] = users.pop()
                new_thread = threading.Thread(
                    target=lambda: start(
                        user_name=user["userName"],
                        password=user["password"],
                        school_id=user["schoolID"], 
                        number=int(user["LessonNumber"]),
                        logger=logger
                    )
                )
                new_thread.run()
                threads.append(new_thread)
                
    
