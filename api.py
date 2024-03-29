import json
from datetime import datetime
from typing import Any, Callable
import requests

from errors import GetArticlesError, GetClassIDError, GetEssayAnswerError, GetSchoolIDError, HttpRequestError, LoginError, ReadEssayError, SubmitEssayTestError


def __post(
        parm: str, 
        jyh: str, 
        exception: Callable[[str], Exception] 
            = lambda msg: HttpRequestError(msg = msg),
        check_result_code: bool = True,
        ) -> Any:
    """Post请求
    对request.post的简单封装, 已携带Headers以及其余所需不变参数, 并检查response状态码是否为200, 检查响应体中result是否为空及是否出现异常

    Args:
        parm (str): 请求参数
        jyh (str): 用于区分业务
        exception (_type_, optional): 错误发生时所需抛出的异常, 默认HttpRequestError
        check_result_code (bool): 是否启用对响应体中result.Code的检测(非1)

    Raises:
        Exception: 调用时确定, 默认为HttpRequestError

    Returns:
        Any: _description_
    """    
    URL = "http://englishservice.siboenglish.com//MobService/index"
    HEADERS = {
        "Content-Type": "application/x-www-form-urlencoded",
        "Accept-Encoding": "gzip",
        "User-Agent": "okhttp/3.12.12"
    }
    data = {
        "jyh": jyh,
        "parm": parm,
        "sign": "",
        "ts": ""
    }
    req = requests.post(url=URL, data=data, headers=HEADERS)
    if req.status_code != 200:
        raise exception(f"Error occoured when contact with server. status code: {req.status_code}")
    req_data = req.json()
    if not req_data['result']:
        raise exception("Unknown Error")
    req_data = json.loads(req_data['result'])
    if check_result_code and req_data["Code"] != "1":
        raise exception(f"[{req_data['Code']}] {req_data['Msg']}")
    return req_data


def login(login_name: str, password: str, school_id: str, app_version: str = "4.5.0") -> dict[str, str]:
    """学生登录

    Args:
        login_name (str): 用户名
        password (str):密码
        school_id (str): 学校id 可由get_school_id确定
        app_version (str, optional): 作用未知 默认为"4.5.0"(抓包值).

    Raises:
        LoginError

    Returns:
        str: user_id 鉴权token
    """

    parm = {
        "schoolID": school_id,
        "loginName": login_name,
        "password": password,
        "ts": 2, # 由抓包获取， 暂时作用未知
        "appVersion": app_version
    }

    loginResult = __post(str(parm), "4002_01", lambda msg: LoginError(msg))
    return {
        "user_name": loginResult["Data"]["UserName"],
        "school_name": loginResult["Data"]["SchoolName"],
        "user_id": loginResult["Data"]["ID"]
    }


def get_school_id(school_name: str) -> str:
    """查找学校schoolID

    Args:
        school_name (str): 学校名称

    Raises:
        GetSchoolIDError

    Returns:
        str: 学校schoolID
    """

    parm = {
        "keyWord": school_name,
        "pageStart": 0,
        "pageSize": 2147483647
    }
    result = __post(str(parm), "4001", lambda msg: GetSchoolIDError(msg))
    
    for school in result['Data']:
        if school["SchoolName"] is school_name: return school["ID"]
    raise GetSchoolIDError(f"Could not find {school_name}.")


def get_classes_id(userID: str) -> list[Any]:
    """获取学生所属所有班级的classID

    Args:
        userID (str): 鉴权token 可通过login获取

    Raises:
        GetClassIDError

    Returns:
        list[Any]: 班级名及classID ig [{"name": "example", "id": "example"}, ...]
    """    
    parm = {
        "userID": userID,
        "ts": 2
    }
    result = __post(str(parm), "1001", lambda msg: GetClassIDError(msg))

    return [
        {"name": Class["ClassName"], "id": Class["ClassID"]}
        for Class in result['Data']
    ]
    

def get_articles(user_id: str, class_id: str, grade: int = 0, length: int = 2147483647) -> list[Any]:
    """获取指定数量的文章

    Args:
        user_id (str): 鉴权token 可通过login获取
        class_id (str): 可通过get_classes_id获取
        grade (int, optional): 文章最低难度. 默认为0
        length (int, optional): 获取最大数量. 默认为2147483647.

    Raises:
        GetArticlesError

 
    Returns:
        list[Any]: 文章的标题、ID及难度, i.g. [{'title': 'example', 'id': 'example', 'grade': 0}, ......]
    """    
    parm = {
        "keyWord": "",
        "eassyType": "",
        "grade": grade,
        "orderType": 1,
        "pageStart": 0,
        "pageSize": length,
        "ts": 2,
        "userID": user_id,
        "classID": class_id
    }


    result: Any = __post(str(parm), "2002", lambda msg: GetArticlesError(msg))
    
    return [
        {'title': article['Title'], "id": article['EssayID'], "grade": article['Grade']}
        for article in result['Data']
    ]


def get_essay_answer(essay: Any) -> str:
    """获取可直接提交形式的文章练习答案

    Args:
        essay_id (str): 文章id, 可由get_articles批量获取

    Raises:
        GetEssayAnswerError

 
    Returns:
        str: 可直接提交形式的文章练习答案
    """
    parm = {
        "essayID": essay['id']
    }
    
    result = __post(str(parm), "2009", lambda msg: GetEssayAnswerError(msg = msg, essay = essay))
    return "".join([
        f"{testcase['TestItemNumber']}-{testcase['Answer']};"
        for testcase in result['Data']
    ])[:-1]


def submit_essay_test(
        user_id: str, 
        essay: Any,
        class_id: str,
        answer: str, 
        create_time: str = datetime.now().replace(microsecond=0).isoformat()
        ):
    parm = {
        "essayID": essay["id"],
        "userID": user_id,
        "classID": class_id,
        "createTime": create_time,
        "itemResult": answer
    }

    __post(str(parm), "2010", lambda msg: SubmitEssayTestError(msg = msg,  essay = essay))

def read_essay(user_id: str, essay: Any, class_id: str):
    parm = {
        "essayID": essay["id"],
        "userID": user_id,
        "classID": class_id
    }
    
    __post(str(parm), "2003", lambda msg: ReadEssayError(msg, essay = essay))