package main

import "github.com/gin-gonic/gin"

func cors(ctx *gin.Context) {
	allow := ctx.Request.Header.Get("Origin")
	ctx.Header("Access-Control-Allow-Origin", allow)
	ctx.Header("Access-Control-Allow-Credentials", "true")
	ctx.Header("Access-Control-Allow-Headers", "Content-Type,Access-Token")
}

func main() {
	r := gin.Default()
	r.Use(cors)
	r.GET("/test", func(c *gin.Context) {
		c.JSON(200, gin.H{"message": "hello world"})
	})
	r.Run(":8081")
}
