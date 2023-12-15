import { BrowserRouter, Route, Routes,  } from "react-router-dom";
import RegisterPage from "./pages/auth/Register";
import LoginPage from "./pages/auth/Login";
import TodoList from "./pages/home/Home";
import TestApp from "./pages/home/Test";



function App() {
 

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/register" element={<RegisterPage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/" element={<TodoList />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
