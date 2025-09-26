# Stellar Europe

A comprehensive platform for building and managing the Stellar blockchain community across Europe. This application enables Ambassadors and Chapter Leads to sign up, organize events, and foster the growth of the Stellar ecosystem throughout the continent.

## 🌟 Features

### User Roles
- **Ambassadors**: Community representatives who organize local meetups, workshops, and educational sessions
- **Chapter Leads**: Regional coordinators who manage large-scale events and strategic initiatives

### Core Functionality
- **User Registration**: Comprehensive signup system with role selection
- **Event Management**: Create, organize, and discover community events
- **Community Hub**: Connect with other Stellar enthusiasts across Europe
- **Professional UI**: Modern, responsive design with Stellar-themed styling

## 🏗️ Architecture

This is a full-stack Rust application built with:

### Backend
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: Role-based user system
- **API**: RESTful JSON API

### Frontend
- **Framework**: Yew (Rust WebAssembly framework)
- **Styling**: Custom CSS with Stellar theme
- **Routing**: Client-side routing with yew-router
- **Build**: Trunk (WASM build tool)

### Shared
- **Data Types**: Common DTOs and types shared between frontend and backend
- **Serialization**: Serde for JSON handling

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- PostgreSQL database
- Trunk (for frontend builds): `cargo install trunk`

### Database Setup
1. Create a PostgreSQL database named `stellareurope`
2. Update the `DATABASE_URL` in `.env` file
3. Run migrations (automatically handled by the backend on startup)

### Running the Application

#### Backend (Port 8080)
```bash
cd backend
cargo run
```

#### Frontend (Port 8000)
```bash
cd frontend
trunk serve
```

The application will be available at:
- Frontend: http://127.0.0.1:8000
- Backend API: http://127.0.0.1:8080

## 🛠️ Development

### Project Structure
```
stellareurope/
├── backend/           # Axum server
│   ├── src/
│   │   ├── database/  # Models, repositories, migrations
│   │   ├── handlers/  # API route handlers
│   │   └── main.rs    # Server entry point
│   └── migrations/    # SQL migration files
├── frontend/          # Yew frontend
│   ├── src/
│   │   ├── components/# UI components
│   │   ├── pages/     # Page components
│   │   ├── routing/   # Route definitions
│   │   └── services/  # API client
│   └── assets/        # Static assets
├── shared/            # Common types and DTOs
└── .env              # Environment configuration
```

### API Endpoints
- `POST /api/signup` - User registration
- `POST /api/events` - Create new event
- `GET /api/events` - List events
- `GET /health` - Health check

### Environment Configuration
Create a `.env` file with:
```
DATABASE_URL=postgresql://postgres:password@localhost:5432/stellareurope
RUST_LOG=info
```

## 🎨 Design System

The application uses a professional dark theme with Stellar-inspired colors:
- **Primary**: `#00d4ff` (Stellar blue)
- **Secondary**: `#ff6b35` (Chapter Lead orange)
- **Background**: Dark gradient from `#000000` to `#1a1a1a`
- **Text**: White and light gray variants

## 📱 Features Walkthrough

### 1. Homepage
- Hero section with clear value proposition
- Feature highlights for both user roles
- Community statistics
- Call-to-action buttons for signup

### 2. Signup System
- Role selection (Ambassador vs Chapter Lead)
- Comprehensive form with validation
- Stellar wallet integration
- Organization and bio fields
- Real-time backend integration

### 3. Navigation
- Role-based navigation options
- Professional styling with hover effects
- Responsive design for mobile
- Clear hierarchy and user flow

### 4. Event Management
- **Event Creation**: Full-featured form for organizing events with comprehensive KPI planning
- **KPI Planning**: Strategic focus areas, quarterly goals, and measurable impact tracking
- **Event Discovery**: Grid layout showing upcoming events
- **Event Types**: Workshop, Meetup, Conference, Hackathon, Community
- **Contact Integration**: Direct organizer contact
- **Strategic Impact**: Connect events to quarterly budget reviews and program goals

### 5. About Page
- Mission and vision statement
- Stellar benefits explanation
- Detailed role descriptions
- Contact information
- Community guidelines

## 🔧 Technical Implementation

### Type Safety
- Full TypeScript-like experience with Rust
- Shared types between frontend and backend
- Compile-time guarantees for API contracts

### Performance
- WebAssembly for near-native frontend performance
- Optimized database queries with SQLx
- Async/await throughout the stack
- Efficient CSS with minimal bundle size

### Security
- SQL injection prevention with prepared statements
- CORS configuration for cross-origin requests
- Input validation on both client and server
- Role-based access patterns

## 🚀 Deployment

### Docker Support
The application can be containerized for easy deployment:

```bash
# Build and run with Docker Compose
docker-compose up --build
```

### Production Considerations
- Set up SSL/TLS termination
- Configure PostgreSQL with proper security
- Set up monitoring and logging
- Implement backup strategies
- Configure environment-specific settings

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🌐 Community

Join the Stellar Europe community:
- Website: [stellareurope.org](https://stellareurope.org)
- Discord: [Join our Discord](https://discord.gg/stellareurope)
- Twitter: [@StellarEurope](https://twitter.com/stellareurope)

## 🙏 Acknowledgments

- Stellar Development Foundation for the amazing blockchain platform
- The Rust community for excellent tooling and libraries
- European blockchain communities for inspiration and feedback

---

Built with ❤️ for the Stellar community across Europe