// Main menu screen / 主菜单界面
// Landing page with links to play, levels, and info / 包含开始游戏、关卡选择、游戏说明链接的启动页

import React, { useEffect } from "react";
import { Link } from "react-router-dom";

const MainMenu = () => {
  // Pause the simulation when entering the menu / 进入菜单时暂停模拟
  useEffect(() => {
    window.paused = true;
  }, []);

  return (
    <div className="main-menu">
      <div className="main-menu-content">
        <h1 className="game-title">Sandspiel</h1>
        <p className="game-subtitle">落沙模拟游戏</p>
        <div className="menu-buttons">
          <Link to="/" className="menu-btn-link">
            <button className="menu-btn menu-btn-primary">开始游戏</button>
          </Link>
          <Link to="/levels" className="menu-btn-link">
            <button className="menu-btn">选择关卡</button>
          </Link>
          <Link to="/info" className="menu-btn-link">
            <button className="menu-btn">游戏说明</button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default MainMenu;
