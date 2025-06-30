using System;
using System.Collections.Generic;
using Microsoft.AspNetCore.Mvc;
using Application.Services;
using Domain.Models;
using Infrastructure.Data;

namespace Api.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class UserController : ControllerBase
    {
        private readonly IUserService _userService;
        private readonly ILogger _logger;
        private readonly DatabaseContext _context;

        public UserController(IUserService userService, ILogger logger, DatabaseContext context)
        {
            _userService = userService;
            _logger = logger;
            _context = context;
        }

        [HttpGet("{id}")]
        public ActionResult<User> GetUser(int id)
        {
            try
            {
                var user = _userService.GetUser(id);
                return Ok(user);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error getting user");
                return NotFound();
            }
        }

        [HttpPost]
        public ActionResult<User> CreateUser(CreateUserDto dto)
        {
            var user = new User
            {
                Email = dto.Email,
                Name = dto.Name
            };
            
            _context.Users.Add(user);
            _context.SaveChanges();
            
            return CreatedAtAction(nameof(GetUser), new { id = user.Id }, user);
        }
    }

    public interface ILogger
    {
        void LogError(Exception ex, string message);
        void LogInfo(string message);
    }

    public class CreateUserDto
    {
        public string Email { get; set; }
        public string Name { get; set; }
    }
}