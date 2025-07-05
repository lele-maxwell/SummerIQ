import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { API } from "@/types/api";
import { getApiUrl } from "@/lib/api";

// Validation functions
const validateEmail = (email: string): { isValid: boolean; message: string } => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!email) {
    return { isValid: false, message: "Email is required" };
  }
  if (!emailRegex.test(email)) {
    return { isValid: false, message: "Please enter a valid email address" };
  }
  return { isValid: true, message: "" };
};

const validatePassword = (password: string): { isValid: boolean; message: string; strength: 'weak' | 'medium' | 'strong' } => {
  if (!password) {
    return { isValid: false, message: "Password is required", strength: 'weak' };
  }
  
  if (password.length < 8) {
    return { isValid: false, message: "Password must be at least 8 characters long", strength: 'weak' };
  }
  
  const hasUpperCase = /[A-Z]/.test(password);
  const hasLowerCase = /[a-z]/.test(password);
  const hasNumbers = /\d/.test(password);
  const hasSpecialChar = /[!@#$%^&*(),.?":{}|<>]/.test(password);
  
  const criteria = [hasUpperCase, hasLowerCase, hasNumbers, hasSpecialChar];
  const metCriteria = criteria.filter(Boolean).length;
  
  if (metCriteria < 2) {
    return { isValid: false, message: "Password must contain at least 3 of: uppercase, lowercase, numbers, special characters", strength: 'weak' };
  }
  
  if (metCriteria === 2) {
    return { isValid: true, message: "Password strength: Medium", strength: 'medium' };
  }
  
  return { isValid: true, message: "Password strength: Strong", strength: 'strong' };
};

const validateConfirmPassword = (password: string, confirmPassword: string): { isValid: boolean; message: string } => {
  if (!confirmPassword) {
    return { isValid: false, message: "Please confirm your password" };
  }
  if (password !== confirmPassword) {
    return { isValid: false, message: "Passwords do not match" };
  }
  return { isValid: true, message: "" };
};

export function AuthForm({ onSuccess }: { onSuccess: () => void }) {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [loginData, setLoginData] = useState({
    email: "",
    password: "",
  });

  const [signupData, setSignupData] = useState({
    name: "",
    email: "",
    password: "",
    confirmPassword: "",
  });

  // Validation states
  const [emailValidation, setEmailValidation] = useState({ isValid: true, message: "" });
  const [passwordValidation, setPasswordValidation] = useState<{ isValid: boolean; message: string; strength: 'weak' | 'medium' | 'strong' }>({ isValid: true, message: "", strength: 'weak' });
  const [confirmPasswordValidation, setConfirmPasswordValidation] = useState({ isValid: true, message: "" });

  const handleLoginChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLoginData({ ...loginData, [e.target.name]: e.target.value });
  };

  const handleSignupChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setSignupData({ ...signupData, [name]: value });
    
    // Real-time validation
    if (name === 'email') {
      setEmailValidation(validateEmail(value));
    } else if (name === 'password') {
      setPasswordValidation(validatePassword(value));
      // Re-validate confirm password when password changes
      setConfirmPasswordValidation(validateConfirmPassword(value, signupData.confirmPassword));
    } else if (name === 'confirmPassword') {
      setConfirmPasswordValidation(validateConfirmPassword(signupData.password, value));
    }
  };

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(getApiUrl(API.login), {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          email: loginData.email,
          password: loginData.password,
        }),
      });

      if (!response.ok) {
        const error = await response.json();
        setError(error.error || 'Login failed');
        return;
      }

      const data = await response.json();
      localStorage.setItem('token', data.token);
      onSuccess();
    } catch (err) {
      setError('Failed to login. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSignup = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);
    
    // Validate all fields before submission
    const emailValid = validateEmail(signupData.email);
    const passwordValid = validatePassword(signupData.password);
    const confirmPasswordValid = validateConfirmPassword(signupData.password, signupData.confirmPassword);
    
    setEmailValidation(emailValid);
    setPasswordValidation(passwordValid);
    setConfirmPasswordValidation(confirmPasswordValid);
    
    if (!emailValid.isValid || !passwordValid.isValid || !confirmPasswordValid.isValid) {
      setError('Please fix the validation errors before submitting.');
      setIsLoading(false);
      return;
    }
    
    try {
      const response = await fetch(getApiUrl(API.register), {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          full_name: signupData.name,
          email: signupData.email,
          password: signupData.password,
        }),
      });

      if (!response.ok) {
        const error = await response.json();
        setError(error.error || 'Registration failed');
        return;
      }

      const data = await response.json();
      localStorage.setItem('token', data.token);
      onSuccess();
    } catch (err) {
      setError('Failed to register. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const getPasswordStrengthColor = (strength: 'weak' | 'medium' | 'strong') => {
    switch (strength) {
      case 'weak': return 'text-red-500';
      case 'medium': return 'text-yellow-500';
      case 'strong': return 'text-green-500';
    }
  };

  const getPasswordStrengthBg = (strength: 'weak' | 'medium' | 'strong') => {
    switch (strength) {
      case 'weak': return 'bg-red-500';
      case 'medium': return 'bg-yellow-500';
      case 'strong': return 'bg-green-500';
    }
  };

  return (
    <div className="w-full max-w-md mx-auto">
      <Tabs defaultValue="login" className="w-full">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="login">Login</TabsTrigger>
          <TabsTrigger value="signup">Sign Up</TabsTrigger>
        </TabsList>
        {error && (
          <div className="mt-4 p-3 bg-destructive/10 text-destructive text-sm rounded-md">
            {error}
          </div>
        )}
        <TabsContent value="login" className="space-y-4 pt-4">
          <form onSubmit={handleLogin} className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="login-email">Email</Label>
              <Input 
                id="login-email"
                name="email"
                type="email"
                placeholder="you@example.com" 
                value={loginData.email}
                onChange={handleLoginChange}
                required
              />
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label htmlFor="login-password">Password</Label>
                <a className="text-xs text-muted-foreground hover:text-primary" href="#">
                  Forgot password?
                </a>
              </div>
              <Input 
                id="login-password"
                name="password"
                type="password"
                placeholder="••••••••" 
                value={loginData.password}
                onChange={handleLoginChange}
                required
              />
            </div>
            <Button type="submit" className="w-full" disabled={isLoading}>
              {isLoading ? "Signing in..." : "Sign In"}
            </Button>
          </form>
        </TabsContent>

        <TabsContent value="signup" className="space-y-4 pt-4">
          <form onSubmit={handleSignup} className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="signup-name">Full Name</Label>
              <Input 
                id="signup-name"
                name="name"
                placeholder="John Doe" 
                value={signupData.name}
                onChange={handleSignupChange}
                required
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="signup-email">Email</Label>
              <Input 
                id="signup-email" 
                name="email"
                type="email"
                placeholder="you@example.com" 
                value={signupData.email}
                onChange={handleSignupChange}
                required
                className={!emailValidation.isValid ? "border-red-500" : ""}
              />
              {!emailValidation.isValid && (
                <p className="text-sm text-red-500">{emailValidation.message}</p>
              )}
            </div>
            <div className="space-y-2">
              <Label htmlFor="signup-password">Password</Label>
              <Input 
                id="signup-password" 
                name="password"
                type="password"
                placeholder="••••••••" 
                value={signupData.password}
                onChange={handleSignupChange}
                required
                className={!passwordValidation.isValid ? "border-red-500" : ""}
              />
              {signupData.password && (
                <div className="space-y-1">
                  <div className="flex gap-1">
                    <div className={`h-1 flex-1 rounded ${passwordValidation.strength === 'weak' ? 'bg-red-500' : 'bg-gray-200'}`}></div>
                    <div className={`h-1 flex-1 rounded ${passwordValidation.strength === 'medium' ? 'bg-yellow-500' : passwordValidation.strength === 'strong' ? 'bg-green-500' : 'bg-gray-200'}`}></div>
                    <div className={`h-1 flex-1 rounded ${passwordValidation.strength === 'strong' ? 'bg-green-500' : 'bg-gray-200'}`}></div>
                  </div>
                  <p className={`text-sm ${getPasswordStrengthColor(passwordValidation.strength)}`}>
                    {passwordValidation.message}
                  </p>
                </div>
              )}
              <div className="text-xs text-muted-foreground">
                Password must be at least 8 characters and contain 3 of: uppercase, lowercase, numbers, special characters
              </div>
            </div>
            <div className="space-y-2">
              <Label htmlFor="signup-confirm-password">Confirm Password</Label>
              <Input 
                id="signup-confirm-password" 
                name="confirmPassword"
                type="password"
                placeholder="••••••••" 
                value={signupData.confirmPassword}
                onChange={handleSignupChange}
                required
                className={!confirmPasswordValidation.isValid ? "border-red-500" : ""}
              />
              {!confirmPasswordValidation.isValid && (
                <p className="text-sm text-red-500">{confirmPasswordValidation.message}</p>
              )}
            </div>
            <Button 
              type="submit" 
              className="w-full" 
              disabled={isLoading || !emailValidation.isValid || !passwordValidation.isValid || !confirmPasswordValidation.isValid}
            >
              {isLoading ? "Creating account..." : "Create Account"}
            </Button>
          </form>
        </TabsContent>
      </Tabs>
    </div>
  );
}
