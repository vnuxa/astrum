# Table of contents

1. [`MatchUtil`](#matchutil) 
2. [`SwitchUtil`](#switchutil) 

[`source`](https://github.com/vnuxa/astrum/blob/master/src/lua_library/astrum/types/utils/init.lua)

---
# MatchUtil
## Propreties:
>   `arm` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is equal to the condition (`==`) 

see definitions: [`MatchUtil`](#matchutil) 
>   `arm_less_eq` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is less than or equal to the condition (`<=`) 

see definitions: [`MatchUtil`](#matchutil) 
>   `arm_less_than` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is less than to the condition (`<`) 

see definitions: [`MatchUtil`](#matchutil) 
>   `arm_more_eq` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is more than or equal to the condition (`>=`) 

see definitions: [`MatchUtil`](#matchutil) 
>   `arm_more_than` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is more than to the condition (`>`) 

see definitions: [`MatchUtil`](#matchutil) 
>   `collapse` → `fun():any`
>    >   Collapses the entire match arm, returning either a result, the fallback if no match was found or nothing 

>   `default` → `fun(result: any):MatchUtil`
>    >   If none of the arms have a match, it will fallback to this result 

see definitions: [`MatchUtil`](#matchutil) 
## Methods:


---
# SwitchUtil
## Propreties:
>   `case` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is equal to the condition (`==`) 

see definitions: [`SwitchUtil`](#switchutil) 
>   `case_less_eq` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is less than or equal to the condition (`<=`) 

see definitions: [`SwitchUtil`](#switchutil) 
>   `case_less_than` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is less than to the condition (`<`) 

see definitions: [`SwitchUtil`](#switchutil) 
>   `case_more_eq` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is more than or equal to the condition (`>=`) 

see definitions: [`SwitchUtil`](#switchutil) 
>   `case_more_than` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is more than to the condition (`>`) 

see definitions: [`SwitchUtil`](#switchutil) 
>   `collapse` → `fun()`
>    >   Evaluates the found case, if no case was found then evaluate the fallback if it is provided 

>   `default` → `fun(result: fun():any):MatchUtil`
>    >   If none of the arms have a match, it will fallback to this result 

see definitions: [`MatchUtil`](#matchutil) 
## Methods:


---
