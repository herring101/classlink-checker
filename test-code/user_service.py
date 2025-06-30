"""User service module for handling user operations."""

from models.user import User
from database import DatabaseConnection
from auth import AuthenticationService

class UserService:
    """Manages user-related operations."""
    
    def __init__(self):
        self.db = DatabaseConnection()
        self.auth = AuthenticationService()
    
    def create_user(self, email: str, password: str) -> User:
        """Create a new user account."""
        user = User(email=email)
        user.set_password(password)
        self.db.save(user)
        return user
    
    def authenticate(self, email: str, password: str) -> User:
        """Authenticate a user."""
        user = self.db.find_by_email(email)
        if self.auth.verify_password(user, password):
            return user
        raise ValueError("Invalid credentials")

class UserManager:
    """High-level user management."""
    
    def __init__(self, service: UserService):
        self.service = service