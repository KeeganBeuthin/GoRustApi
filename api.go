package main

import (
	"net/http"
	"sync"

	"github.com/gin-gonic/gin"
)

type Book struct {
	ID     uint64 `json:"id"`
	Title  string `json:"title"`
	Author string `json:"author"`
}

type BookStore struct {
	sync.RWMutex
	books []Book
}

func NewBookStore() *BookStore {
	return &BookStore{
		books: make([]Book, 0),
	}
}

func (bs *BookStore) GetBooks() []Book {
	bs.RLock()
	defer bs.RUnlock()
	return bs.books
}

func (bs *BookStore) AddBook(book Book) {
	bs.Lock()
	defer bs.Unlock()
	bs.books = append(bs.books, book)
}

func main() {
	r := gin.Default()
	store := NewBookStore()

	r.GET("/books", func(c *gin.Context) {
		c.JSON(http.StatusOK, store.GetBooks())
	})

	r.POST("/books", func(c *gin.Context) {
		var newBook Book
		if err := c.BindJSON(&newBook); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		store.AddBook(newBook)
		c.Status(http.StatusCreated)
	})

	r.Run(":8080")
}
