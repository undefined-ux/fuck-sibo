from typing import Any
from requests import Response

class HttpRequestError(Exception): 
    def __init__(self, response: Response | None = None, msg: str = "") -> None:
        super().__init__(msg)
        self.msg = msg
        self.reponse = response
    def __str__(self) -> str:
       return self.msg
    
class LoginError(Exception): 
    def __init__(self, msg: str = "") -> None:
        super().__init__(msg)
        self.msg = msg
    def __str__(self) -> str:
       return f"Failed to Login : {self.msg}"
    

class GetSchoolIDError(Exception): 
    def __init__(self, msg: str = "") -> None:
        super().__init__(msg)
        self.msg = msg
    def __str__(self) -> str:
       return f"Failed to get SchoolID: {self.msg}"
    

class GetClassIDError(Exception): 
    def __init__(self, msg: str = "") -> None:
        super().__init__(msg)
        self.msg = msg
    def __str__(self) -> str:
       return f"Failed to get ClassID: {self.msg}"
    
class GetArticlesError(Exception):
    def __init__(self, msg: str = "") -> None:
        super().__init__(msg)
        self.msg = msg
    def __str__(self) -> str:
        return f"Failed to get Articles: {self.msg}"
    
# class GetArticleContentError(Exception):
#     def __init__(self, msg: str = "", essay: Any = {"title": "", "id": "", "grade": 0}) -> None:
#         super().__init__(msg)
#         self.msg = msg
#         self.essay = essay
#     def __str__(self) -> str:
#         return f"Failed to get Essay \"{self.essay['title']}\" Content: {self.msg}"

class GetEssayAnswerError(Exception):
    def __init__(self, msg: str = "", essay: Any = {"title": "", "id": "", "grade": 0}) -> None:
        super().__init__(msg)
        self.msg = msg
        self.essay = essay
    def __str__(self) -> str:
        return f"Failed to get Essay {self.essay['title']} Answer: {self.msg}"
    

class SubmitEssayTestError(Exception):
    def __init__(self, msg: str = "", essay: Any = {"title": "", "id": "", "grade": 0}) -> None:
        super().__init__(msg)
        self.msg = msg
        self.essay = essay
    def __str__(self) -> str:
        return f"Failed to Submit Essay {self.essay['title']}: {self.msg}"