import { BrowserRouter, Route, Routes  } from "react-router-dom";
import TodoList from "./pages/todo";
import RegisterPage from "./pages/register";
import LoginPage from "./pages/login";




function App() {
 

  return (
    <BrowserRouter>
      <Routes>
        
        <Route path="/todo" element={<TodoList />} />
        <Route path="/" element={<RegisterPage />} />
        <Route path="/login" element={<LoginPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
