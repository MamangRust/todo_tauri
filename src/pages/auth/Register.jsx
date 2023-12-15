import { useState } from 'react'
import { Form, Input, Button, Card, Checkbox } from 'antd';

import "./style/register.css"
import { Link } from 'react-router-dom';


const RegisterPage = () => {
  const [form] = Form.useForm();
  const [confirmDirty, setConfirmDirty] = useState(false);

  const onFinish = (values) => {
    console.log('Received values:', values);
    // Lakukan sesuatu dengan nilai-nilai yang diterima dari form
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

  const compareToFirstPassword = (_, value) => {
    const { getFieldValue } = form;
    if (value && value !== getFieldValue('password')) {
      return Promise.reject(new Error('Konfirmasi password tidak sesuai'));
    }
    return Promise.resolve();
  };

  return (
    <div className="center-container">
      <Card className="register-card">
        <h1 className="register-card-title">Register</h1>
        <Form
          form={form}
          name="register"
          onFinish={onFinish}
          onFinishFailed={onFinishFailed}
          layout="vertical"
          className="register-form"
        >
          <Form.Item
            label="Name"
            name="name"
            className='register-form-item'
            rules={[{ required: true, message: 'Please input your name!' }]}
          >
            <Input size="large" placeholder="Enter your name" />
          </Form.Item>

          <Form.Item
            label="Email"
            name="email"
            className='register-form-item'
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
            className='register-form-item'
            rules={[
              { required: true, message: 'Please input your password!' },
              { validator: validateToNextPassword },
            ]}
          >
            <Input.Password size="large" placeholder="Enter your password" />
          </Form.Item>

          <Form.Item
            label="Confirm Password"
            name="confirmPassword"
            className='register-form-item'
            dependencies={['password']}
            rules={[
              { required: true, message: 'Please confirm your password!' },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue('password') === value) {
                    return Promise.resolve();
                  }
                  return Promise.reject(new Error('The two passwords that you entered do not match!'));
                },
              }),
            ]}
          >
            <Input.Password size="large" placeholder="Confirm your password" />
          </Form.Item>

          <Form.Item>
            <Button type="primary" htmlType="submit" size="large" block>
              Register
            </Button>
          </Form.Item>
        </Form>
        <div>
          Already have an account? <Link to="/">Login</Link>
        </div>
      </Card>
    </div>
  );
};

export default RegisterPage;
