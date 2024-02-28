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