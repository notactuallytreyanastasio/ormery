using Microsoft.EntityFrameworkCore;
using TodoApp.Data;
using TodoApp.Models;

var builder = WebApplication.CreateBuilder(args);

// Configure to listen on port 5002
builder.WebHost.UseUrls("http://localhost:5002");

// Add services
builder.Services.AddRazorPages();
builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseSqlite("Data Source=todos.db"));

var app = builder.Build();

// Auto-migrate and seed
using (var scope = app.Services.CreateScope())
{
    var db = scope.ServiceProvider.GetRequiredService<AppDbContext>();
    db.Database.EnsureCreated();

    if (!db.Lists.Any())
    {
        var personal = new TodoList { Name = "Personal", CreatedAt = DateTime.UtcNow };
        var work = new TodoList { Name = "Work", CreatedAt = DateTime.UtcNow };
        db.Lists.AddRange(personal, work);
        db.SaveChanges();

        db.Todos.AddRange(
            new TodoItem { Title = "Buy groceries", Completed = false, ListId = personal.Id, CreatedAt = DateTime.UtcNow },
            new TodoItem { Title = "Walk the dog", Completed = true, ListId = personal.Id, CreatedAt = DateTime.UtcNow },
            new TodoItem { Title = "Read a book", Completed = false, ListId = personal.Id, CreatedAt = DateTime.UtcNow },
            new TodoItem { Title = "Finish quarterly report", Completed = false, ListId = work.Id, CreatedAt = DateTime.UtcNow },
            new TodoItem { Title = "Review pull requests", Completed = true, ListId = work.Id, CreatedAt = DateTime.UtcNow },
            new TodoItem { Title = "Update documentation", Completed = false, ListId = work.Id, CreatedAt = DateTime.UtcNow }
        );
        db.SaveChanges();
    }
}

// Configure pipeline
if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Error");
}

app.UseRouting();
app.UseAuthorization();
app.MapStaticAssets();
app.MapRazorPages()
   .WithStaticAssets();

app.Run();
