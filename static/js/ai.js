// 本地存储键名
const JWT_KEY = 'jwt_token';

// DOM元素
const loginPanel = document.getElementById('loginPanel');
const dashboardPanel = document.getElementById('dashboardPanel');
const tokenPanel = document.getElementById('tokenPanel');
const loginBtn = document.getElementById('loginBtn');
const logoutBtn = document.getElementById('logoutBtn');
const refreshBtn = document.getElementById('refreshBtn');
const tokenDetails = document.getElementById('tokenDetails');
const loginStatus = document.getElementById('loginStatus');
const tabs = document.querySelectorAll('.tab');

// 初始化页面
document.addEventListener('DOMContentLoaded', () => {
    // 检查当前令牌状态
    checkAuth();

    // 设置标签切换事件
    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            // 移除所有活动标签
            tabs.forEach(t => t.classList.remove('active'));
            // 添加活动标签
            tab.classList.add('active');

            // 隐藏所有面板
            document.querySelectorAll('.panel').forEach(panel => {
                panel.classList.remove('active');
            });

            // 显示对应面板
            const tabName = tab.getAttribute('data-tab');
            if (tabName === 'login') {
                loginPanel.classList.add('active');
            } else if (tabName === 'dashboard') {
                dashboardPanel.classList.add('active');
            } else if (tabName === 'token') {
                tokenPanel.classList.add('active');
                updateTokenDisplay();
            }
        });
    });

    // 登录按钮事件
    loginBtn.addEventListener('click', handleLogin);

    // 登出按钮事件
    logoutBtn.addEventListener('click', handleLogout);

    // 刷新令牌按钮事件
    refreshBtn.addEventListener('click', handleRefreshToken);

    // 每5秒检查一次令牌状态
    setInterval(checkAuth, 5000);
});

// 检查认证状态
function checkAuth() {
    const token = getToken();
    if (!token) {
        showPanel('login');
        updateLoginStatus('未检测到令牌，请登录', 'status-missing');
        return false;
    }

    if (isTokenExpired(token)) {
        showPanel('login');
        updateLoginStatus('令牌已过期，请重新登录', 'status-expired');
        return false;
    }

    // 令牌有效
    showPanel('dashboard');
    updateLoginStatus('令牌有效', 'status-valid');
    return true;
}

// 检查JWT是否过期
function isTokenExpired(token) {
    try {
        // 解析JWT负载
        const payload = parseJwtPayload(token);

        // 检查exp字段是否存在
        if (!payload.exp) {
            return true;
        }

        // 获取当前时间（秒）
        const currentTime = Math.floor(Date.now() / 1000);

        // 检查是否过期
        return currentTime >= payload.exp;
    } catch (e) {
        console.error('解析JWT失败:', e);
        return true;
    }
}

// 解析JWT负载
function parseJwtPayload(token) {
    // JWT由三部分组成：header.payload.signature
    const base64Url = token.split('.')[1];

    // 替换Base64 URL编码字符
    const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');

    // 解码Base64
    const jsonPayload = decodeURIComponent(
        atob(base64)
            .split('')
            .map(c => '%' + ('00' + c.charCodeAt(0).toString(16).slice(-2))
                .join('')
            ));

    return JSON.parse(jsonPayload);
}

// 获取令牌
function getToken() {
    return localStorage.getItem(JWT_KEY);
}

// 保存令牌
function saveToken(token) {
    localStorage.setItem(JWT_KEY, token);
}

// 删除令牌
function clearToken() {
    localStorage.removeItem(JWT_KEY);
}

// 处理登录
function handleLogin() {
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const expiresIn = parseInt(document.getElementById('expiresIn').value) || 30;

    if (!username || !password) {
        updateLoginStatus('请输入用户名和密码', 'status-missing');
        return;
    }

    // 模拟登录请求
    simulateLogin(username, password, expiresIn)
        .then(response => {
            if (response.success) {
                saveToken(response.token);
                updateLoginStatus('登录成功！正在重定向...', 'status-valid');
                setTimeout(() => {
                    showPanel('dashboard');
                    updateTokenDisplay();
                }, 1000);
            } else {
                updateLoginStatus('登录失败: ' + response.message, 'status-expired');
            }
        });
}

// 处理登出
function handleLogout() {
    clearToken();
    showPanel('login');
    updateLoginStatus('您已成功登出', 'status-missing');
}

// 处理令牌刷新
function handleRefreshToken() {
    const token = getToken();
    if (!token) {
        updateLoginStatus('没有可刷新的令牌', 'status-missing');
        return;
    }

    // 模拟刷新令牌请求
    simulateRefreshToken(token)
        .then(response => {
            if (response.success) {
                saveToken(response.token);
                updateTokenDisplay();
                updateLoginStatus('令牌刷新成功！', 'status-valid');
            } else {
                updateLoginStatus('令牌刷新失败: ' + response.message, 'status-expired');
            }
        });
}

// 模拟登录API
function simulateLogin(username, password, expiresIn) {
    return new Promise(resolve => {
        setTimeout(() => {
            // 简单验证
            if (username === 'admin' && password === 'password123') {
                // 创建模拟JWT
                const payload = {
                    sub: '1234567890',
                    name: 'Admin User',
                    iat: Math.floor(Date.now() / 1000),
                    exp: Math.floor(Date.now() / 1000) + expiresIn
                };

                // 模拟JWT格式
                const token = `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.${btoa(JSON.stringify(payload))}.simulated_signature`;

                resolve({
                    success: true,
                    token: token,
                    message: '登录成功'
                });
            } else {
                resolve({
                    success: false,
                    message: '用户名或密码无效'
                });
            }
        }, 800);
    });
}

// 模拟刷新令牌API
function simulateRefreshToken(oldToken) {
    return new Promise(resolve => {
        setTimeout(() => {
            try {
                const payload = parseJwtPayload(oldToken);

                // 创建新令牌（延长30秒）
                const newPayload = {
                    ...payload,
                    iat: Math.floor(Date.now() / 1000),
                    exp: Math.floor(Date.now() / 1000) + 30
                };

                // 模拟JWT格式
                const token = `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.${btoa(JSON.stringify(newPayload))}.simulated_signature`;

                resolve({
                    success: true,
                    token: token,
                    message: '令牌刷新成功'
                });
            } catch (e) {
                resolve({
                    success: false,
                    message: '刷新令牌失败'
                });
            }
        }, 800);
    });
}

// 更新令牌显示
function updateTokenDisplay() {
    const token = getToken();
    if (!token) {
        tokenDetails.innerHTML = '没有可用的令牌';
        return;
    }

    try {
        const payload = parseJwtPayload(token);
        const currentTime = Math.floor(Date.now() / 1000);
        const expiresIn = payload.exp - currentTime;
        const isExpired = expiresIn <= 0;

        let html = `<div><span class="status-indicator ${isExpired ? 'status-expired' : 'status-valid'}"></span>`;
        html += isExpired ? '<strong>令牌已过期</strong>' : '<strong>令牌有效</strong>';
        html += '</div>';

        html += `<div class="token-expiration">`;
        if (isExpired) {
            html += `过期时间: ${new Date(payload.exp * 1000).toLocaleString()}`;
        } else {
            html += `将在 <strong>${expiresIn}</strong> 秒后过期`;
        }
        html += `</div>`;

        html += `<div>签发时间: ${new Date(payload.iat * 1000).toLocaleString()}</div>`;
        html += `<div>过期时间: ${new Date(payload.exp * 1000).toLocaleString()}</div>`;
        html += `<div>用户: ${payload.name}</div>`;

        tokenDetails.innerHTML = html;
    } catch (e) {
        tokenDetails.innerHTML = '解析令牌失败';
    }
}

// 更新登录状态
function updateLoginStatus(message, statusClass) {
    const statusIndicator = statusClass ?
        `<span class="status-indicator ${statusClass}"></span>` : '';
    loginStatus.innerHTML = `${statusIndicator}${message}`;
}

// 显示指定面板
function showPanel(panelName) {
    // 隐藏所有面板
    loginPanel.classList.remove('active');
    dashboardPanel.classList.remove('active');
    tokenPanel.classList.remove('active');

    // 移除所有活动标签
    tabs.forEach(t => t.classList.remove('active'));

    // 显示指定面板
    if (panelName === 'login') {
        loginPanel.classList.add('active');
        document.querySelector('.tab[data-tab="login"]').classList.add('active');
    } else if (panelName === 'dashboard') {
        dashboardPanel.classList.add('active');
        document.querySelector('.tab[data-tab="dashboard"]').classList.add('active');
    } else if (panelName === 'token') {
        tokenPanel.classList.add('active');
        document.querySelector('.tab[data-tab="token"]').classList.add('active');
    }
}