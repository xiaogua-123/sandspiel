// React Router application setup / React Router应用配置
// Defines all routes and mounts the React tree into the DOM / 定义所有路由并将React树挂载到DOM中

import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route } from "react-router-dom";

import Info from "./components/info";
import { Index } from "./components/ui";
import Browse from "./components/browse";
import Admin from "./components/admin";
import Menu from "./components/menu";
import SignInScreen from "./components/signin";
import BenchmarkRunner from "./components/benchmarkRunner";
import MainMenu from "./components/MainMenu";
import LevelSelect from "./components/LevelSelect";

function BrowseRouter({ match, location }) {
  return (
    <Menu>
      <Browse location={location} />
    </Menu>
  );
}

function AdminRouter({ match, location }) {
  return (
    <Menu>
      <Admin location={location} />
    </Menu>
  );
}

function SigninRouter({ match, location }) {
  return (
    <Menu>
      <SignInScreen location={location} />
    </Menu>
  );
}

// Top-level router with all page routes / 顶层路由器，包含所有页面路由
function AppRouter() {
  return (
    <Router>
      <Route exact path="/menu" component={MainMenu} />
      <Route exact path="/levels" component={LevelSelect} />

      <Route exact path="/" component={Index} />
      <Route
        exact
        path="/info/"
        render={() => (
          <Menu>
            <Info />
          </Menu>
        )}
      />
      <Route exact path="/bench" component={BenchmarkRunner} />
      <Route path="/browse" component={BrowseRouter} />
      <Route path="/admin" component={AdminRouter} />
      <Route path="/login" component={SigninRouter} />
      <Route path="/__/auth/handler" component={SigninRouter} />
    </Router>
  );
}

ReactDOM.render(<AppRouter />, document.getElementById("ui"));
