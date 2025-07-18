'use strict';

const JWT_KEY = 'jwt_token';

// 不需要鉴权的API前缀
const UNPROTECTED_API = [
    "/index.html",
    "/login",
    "/"
];

/**
 * 使用用户名和密码登录, 此函数将会把密码进行哈希, 以保护用户数据安全, 故传入的密码不需要哈希
 * @param {string} id 用户标示
 * @param {string} password 用户密码
 * @returns true 如果成功
 * @returns false 如果失败
 * @throws 如果 fetch 函数失败
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
            return true;
        } else {
            return Promise.reject(token);
        }
    } catch (error) {
        throw error;
    }
}

/**
 * 将原有的请求封装一个 Authorization 字段, 实现自动携带 token
 * @param {string} url 请求的url
 * @param {object} options 其他的请求报文选项
 * @returns 服务器的相应
 * @throws 当 fetch 函数出现问题时
 */
export async function auth_fetch(url, options = {}) {
    const token = get_token();

    const headers = new Headers(options.headers || {});
    if (token) {
        headers.set("Authorization", `Bearer ${token}`);
    } else {
        return Promise.reject(new Error("token 不存在!"));
    }

    try {
        const response = await fetch(url, {
            ...options,
            headers
        });

        if (response.status == 401) {
            clear_token();
            return Promise.reject(await response.json());
        } else {
            return response;
        }
    } catch (error) {
        throw error;
    }
}

/**
 * 检查当前路径是否需要认证
 * @returns true 如果需要鉴权, false 如果不需要鉴权
 */
export function requires_auth() {
    const path = window.location.pathname;
    if (UNPROTECTED_API.includes(path)) {
        return false;
    } else {
        return true;
    }
}

/**
 * 将 token 存储到本地
 * @param {string} token 需要存储的 token
 */
export function save_token(token) {
    localStorage.setItem(JWT_KEY, token);
}

/**
 * 清除本地存储的 jwt
 */
export function clear_token() {
    localStorage.removeItem(JWT_KEY);
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