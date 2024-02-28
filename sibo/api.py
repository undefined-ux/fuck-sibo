import json
from datetime import datetime
from typing import Any, Callable
import requests


from sibo.errors import GetClassIDError, GetSchoolIDError, HttpRequestError, LoginError


def __post(
        parm: str, 
        jyh: str, 
        exception: Callable[[str], Exception] 
            = lambda msg: HttpRequestError(msg = msg)
        ) -> Any:
    """Post请求
    对request.post的简单封装, 已携带Headers以及其余所需不变参数, 并检查response状态码是否为200, 检查响应体中result是否为空及是否出现异常

    Args:
        parm (str): 请求参数
        jyh (str): 用于区分业务
        exception (_type_, optional): 错误发生时所需抛出的异常, 默认抛出HttpRequestError

    Raises:
        exception: _description_
        exception: _description_
        exception: _description_

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
    if req.status_code is not 200:
        raise exception(f"Error occoured when contact with server. status code: {req.status_code}")
    req_data = req.json()
    if not req_data['result']:
        raise exception("Unknown Error")
    req_data = json.loads(req_data['result'])
    if req_data["Code"] is not 1:
        raise exception(f"[{req_data['Code']}] {req_data['Msg']}")
    return req_data['result']


def login(login_name: str, password: str, school_id: str, app_version: str = "4.5.0") -> str:
    """学生登录

    Args:
        login_name (str): 用户名
        password (str):密码
        school_id (str): 学校id 可由get_school_id确定
        app_version (str, optional): 作用未知 默认为"4.5.0"(抓包值).

    Raises:
        LoginError: 登录失败时抛出

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
    return loginResult['Data']['ID']


def get_school_id(school_name: str) -> str:
    """查找学校schoolID

    Args:
        school_name (str): 学校名称

    Raises:
        GetSchoolIDError: 查找失败或未找到学校时抛出

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


def get_classes_id(userID: str) -> list[object]:
    """获取学生所属所有班级的classID

    Args:
        userID (str): 鉴权token 可通过login获取

    Returns:
        list[object]: 班级名及classID ig [{"name": "example", "classID": "example"}, ...]
    """    
    parm = {
        "userID": userID,
        "ts": 2
    }
    result = __post(str(parm), "1001", lambda msg: GetSchoolIDError(msg))
    
    