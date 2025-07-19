"use strict";

/**
 * 将参数对象转换为请求参数字符串
 * @param {object} param_obj 参数对象
 * @returns 参数对象转化为的请求参数字符串
 */
export function param_string(param_obj) {
    const param_str = Object.keys(param_obj)
        .filter((key) => param_obj[key].length > 0)
        .map((key) => key + "=" + param_obj[key])
        .join("&");
    
    if (param_str.length > 0) {
        return "?" + param_str;
    } else {
        return "";
    }
}
