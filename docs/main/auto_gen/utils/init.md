# Table of contents

1. [`MatchUtil`](#MatchUtil) 
2. [`SwitchUtil`](#SwitchUtil) 

---
# MatchUtil
## Propreties:
>   `arm` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is equal to the condition (`==`) 

see definitions: [`MatchUtil`](#MatchUtil) 
>   `arm_less_eq` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is less than or equal to the condition (`<=`) 

see definitions: [`MatchUtil`](#MatchUtil) 
>   `arm_less_than` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is less than to the condition (`<`) 

see definitions: [`MatchUtil`](#MatchUtil) 
>   `arm_more_eq` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is more than or equal to the condition (`>=`) 

see definitions: [`MatchUtil`](#MatchUtil) 
>   `arm_more_than` → `fun(condition: any, result: any):MatchUtil`
>    >   Checks if the `scrutinee` is more than to the condition (`>`) 

see definitions: [`MatchUtil`](#MatchUtil) 
>   `collapse` → `fun():any`
>    >   Collapses the entire match arm, returning either a result, the fallback if no match was found or nothing 

>   `default` → `fun(result: any):MatchUtil`
>    >   If none of the arms have a match, it will fallback to this result 

see definitions: [`MatchUtil`](#MatchUtil) 
## Methods:


---
# SwitchUtil
## Propreties:
>   `case` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is equal to the condition (`==`) 

see definitions: [`SwitchUtil`](#SwitchUtil) 
>   `case_less_eq` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is less than or equal to the condition (`<=`) 

see definitions: [`SwitchUtil`](#SwitchUtil) 
>   `case_less_than` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is less than to the condition (`<`) 

see definitions: [`SwitchUtil`](#SwitchUtil) 
>   `case_more_eq` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is more than or equal to the condition (`>=`) 

see definitions: [`SwitchUtil`](#SwitchUtil) 
>   `case_more_than` → `fun(condition: any, result: fun():any):SwitchUtil`
>    >   Checks if the `scrutinee` is more than to the condition (`>`) 

see definitions: [`SwitchUtil`](#SwitchUtil) 
>   `collapse` → `fun()`
>    >   Evaluates the found case, if no case was found then evaluate the fallback if it is provided 

>   `default` → `fun(result: fun():any):MatchUtil`
>    >   If none of the arms have a match, it will fallback to this result 

see definitions: [`MatchUtil`](#MatchUtil) 
## Methods:


---
