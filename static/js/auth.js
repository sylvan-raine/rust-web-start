// 本地存储的JWT键名
export const JWT_KEY = 'jwt_token';

// 不需要鉴权的API前缀
const UNPROTECTED_API = [
    "/index.html",
    "/login",
    "/"
];

/**
 * 检查当前路径是否需要认证
 * @returns true 如果需要鉴权, false 如果不需要鉴权
 */
function requires_auth() {
    const path = window.location.pathname;
    return !(UNPROTECTED_API.indexOf(path) >= 0);
}

/**
 * 获取 jwt
 * @returns token 如果本地存储了 jwt 的话, 否则将为 null
 */
export function get_token() {
    return localStorage.getItem(JWT_KEY);
}

/**
 * 将 token 存储到本地
 * @param {*} token 需要存储的 token
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
 * 检查jwt的存在状态
 * @returns false 如果需要鉴权但是 token 不存在
 */
export function check_auth() {
    if (requires_auth() && !get_token()) {
        redirect_login();
        return false;
    }
    return true;
}

/**
 * 重定向到登录页
 */
export function redirect_login() {
    window.location.href = '/login.html';
}

/**
 * 将原有的请求封装一个 Authorization 字段, 实现自动携带 token
 * @param {*} url 请求的url
 * @param {*} options 其他的请求报文选项
 * @returns 服务器的相应
 */
export async function auth_fetch(url, options = {}) {
    const token = get_token();

    // 设置认证头
    const headers = new Headers(options.headers || {});
    if (token) headers.set('Authorization', `Bearer ${token}`);

    try {
        const response = await fetch(url, {
            ...options,
            headers
        });

        // 处理401未授权
        if (response.status === 401) {
            clear_token();
            redirect_login();
            return Promise.reject(new Error('token 过期啦, 或者说被人为更改了, 反正就是用不了了'));
        } else {
            return response;
        }
    } catch (error) {
        console.error('不知道啥失败了:', error);
        throw error;
    }
}