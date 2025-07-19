'use strict';

const JWT_KEY = 'jwt_token';

/**
 * 使用用户名和密码登录, 此函数将会把密码进行哈希, 以保护用户数据安全, 故传入的密码不需要哈希
 * @param {string} id 用户标示
 * @param {string} password 用户密码
 * @returns 如果成功, 什么都不会返回
 * @throws response, 如果服务器返回了错误信息, 或者 fetch 函数出错了会抛出异常
 */
export async function login(id, password) {
    const data = {
        id, password
    };

    const json = JSON.stringify(data);

    try {
        const response = await fetch("/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: json
        });

        const token = await response.json();
        if (response.ok) {
            localStorage.setItem(JWT_KEY, token);
            return;
        } else {
            throw response;
        }
    } catch (error) {
        throw error;
    }
}

/**
 * 推出登录, 通过清除 token 实现
 */
export function logout() {
    localStorage.removeItem(JWT_KEY);
}

/**
 * 将原有的请求封装一个 Authorization 字段, 实现自动携带 token
 * @param {string} method 请求方法
 * @param {string} url 请求的路径
 * @param {object | Headers} options 其他的请求报文选项
 * @returns 服务器响应的 Promise, 毕竟这个函数就只是多添加一个请求头, 当然如果说本地没有 token, 那就会直接返回错误
 */
export async function auth_fetch(method, url, options = {}) {
    const headers = new Headers(options);
    const token = get_token();
    if (token) {
        headers.set("Authorization", `Bearer ${token}`);
    } else {
        throw new Error("试图在 token 不存在的情况下请求受保护的 API!");
    }

    return fetch(url, { method, headers });
}

/**
 * 因为本地时间和服务器时间不同步, 会有一些时差, 如果时差很大, 就不能如实反映登陆状态
 * @returns true, 如果 token 有大概率合法, false, 如果 token 有大概率不合法
 */
export function token_legal() {
    const token = get_token();
    if (token) {
        const pay_load = token.split(".")[1];
        const prev_feedbk = JSON.parse(atob(pay_load));
        if (prev_feedbk) {
            if (Number(prev_feedbk.exp) >= Date.now() / 1000) {
                return true;
            } else {
                // console.log("jwt 有可能已过期!");
                return false;
            }
        } else {
            // console.log("服务器未返回一个合法的 jwt!");
            return false;
        }
    } else {
        // console.log("jwt 不存在!");
        return false;
    }
}

function get_token() {
    return localStorage.getItem(JWT_KEY);
}
