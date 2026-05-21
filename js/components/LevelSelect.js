// Level selection grid / 关卡选择网格
// Displays all levels as cards with difficulty ratings / 以卡片形式展示所有关卡及其难度评级

import React from "react";
import { Link } from "react-router-dom";
import levels from "../game/levels";

// Render star rating string from difficulty level (1-5) / 根据难度等级(1-5)渲染星级字符串
const difficultyStars = (n) => "★".repeat(n) + "☆".repeat(5 - n);

const LevelSelect = () => {
  const grid = levels.map((level) => (
    <Link
      to={`/?level=${level.id}`}
      key={level.id}
      className="level-card-link"
    >
      <div className={`level-card level-diff-${level.difficulty}`}>
        <div className="level-id">{level.id}</div>
        <div className="level-info">
          <div className="level-name">{level.name}</div>
          <div className="level-desc">{level.description}</div>
          <div className="level-diff">{difficultyStars(level.difficulty)}</div>
        </div>
      </div>
    </Link>
  ));

  return (
    <div className="level-select-scrim">
      <div className="level-select">
        <div className="level-select-header">
          <h2>选择关卡</h2>
          <Link to="/menu" className="x">
            <button>x</button>
          </Link>
        </div>
        <div className="level-grid">{grid}</div>
      </div>
    </div>
  );
};

export default LevelSelect;
