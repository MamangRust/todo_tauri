import { useState } from 'react';
import { Form, Input, Button, Card, Checkbox } from 'antd';
import "./style/login.css"
import { Link } from 'react-router-dom'

const LoginPage = () => {
    const [form] = Form.useForm();
    const [confirmDirty, setConfirmDirty] = useState(false);

    const onFinish = (values) => {
        console.log('Received values:', values);
        // Perform actions with the received form values for login
    };

    const onFinishFailed = (errorInfo) => {
        console.log('Failed:', errorInfo);
    };

    const handleConfirmBlur = (e) => {
        const { value } = e.target;
        setConfirmDirty(confirmDirty || !!value);
    };

    const validateToNextPassword = (_, value) => {
        const { validateFields } = form;
        if (value && confirmDirty) {
            validateFields(['confirm'], { force: true });
        }
        return Promise.resolve();
    };

    return (
        <div className="center-container">
            <Card className="login-card">
                <h1 className="login-card-title">Login</h1>
                <Form
                    form={form}
                    name="login"
                    onFinish={onFinish}
                    onFinishFailed={onFinishFailed}
                    layout="vertical"
                    className="login-form"
                >
                    <Form.Item
                        label="Email"
                        name="email"
                        className='login-form-item'
                        rules={[
                            { required: true, message: 'Please input your email!' },
                            { type: 'email', message: 'Invalid email format!' },
                        ]}
                    >
                        <Input size="large" placeholder="Enter your email" />
                    </Form.Item>

                    <Form.Item
                        label="Password"
                        name="password"
                        rules={[
                            { required: true, message: 'Please input your password!' },
                            { validator: validateToNextPassword },
                        ]}
                        className='login-form-item'
                    >
                        <Input.Password size="large" placeholder="Enter your password" />
                    </Form.Item>

                    <Form.Item>
                        <Checkbox>Remember me</Checkbox>
                        <Link to="/forgot-password">Lost Password?</Link>
                        <Button type="primary" htmlType="submit" size="large" block>
                            Login
                        </Button>
                    </Form.Item>
                </Form>
                <div>
                    Not registered? <Link to="/register">Create Account</Link>
                </div>
            </Card>
        </div>
    );
};

export default LoginPage;
