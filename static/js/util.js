"use strict";

/**
 * 处理 url, 将传入的参数对象序列化为url的参数, 最后和 url 进行拼接, 完成 url 的处理
 * @param {string} url 基础的不带参数的 url
 * @param {object} param_obj 参数对象
 * @returns 拼接后的带参数的 url
 */
export function url_with_param(url, param_obj) {
    const param_str = Object.keys(param_obj)
        .filter((key) => param_obj[key].length > 0)
        .map((key) => key + "=" + param_obj[key])
        .join("&");
    
    if (param_str.length > 0) {
        return url + "?" + param_str;
    } else {
        return url;
    }
}