package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/knakk/rdf"
)

var triples []rdf.Triple

type InputTriple struct {
	Subject   string `json:"subject"`
	Predicate string `json:"predicate"`
	Object    string `json:"object"`
}

type JSONTerm struct {
	Type  string `json:"type"`
	Value string `json:"value"`
}

type JSONTriple struct {
	Subject   JSONTerm `json:"subject"`
	Predicate JSONTerm `json:"predicate"`
	Object    JSONTerm `json:"object"`
}

func main() {
	r := gin.Default()

	r.GET("/triples", getTriples)
	r.POST("/triples", addTriple)

	r.Run(":8080")
}

func getTriples(c *gin.Context) {
	jsonTriples := make([]InputTriple, len(triples))
	for i, t := range triples {
		jsonTriples[i] = InputTriple{
			Subject:   t.Subj.String(),
			Predicate: t.Pred.String(),
			Object:    t.Obj.String(),
		}
	}
	c.JSON(http.StatusOK, jsonTriples)
}

func termToJSON(term rdf.Term) JSONTerm {
	switch v := term.(type) {
	case rdf.IRI:
		return JSONTerm{Type: "IRI", Value: v.String()}
	case rdf.Literal:
		return JSONTerm{Type: "Literal", Value: v.String()}
	default:
		return JSONTerm{Type: "Unknown", Value: term.String()}
	}
}

func addTriple(c *gin.Context) {
	var input InputTriple

	if err := c.ShouldBindJSON(&input); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	subj, err := rdf.NewIRI(input.Subject)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid subject IRI"})
		return
	}

	pred, err := rdf.NewIRI(input.Predicate)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid predicate IRI"})
		return
	}

	var obj rdf.Object
	if isValidIRI(input.Object) {
		obj, err = rdf.NewIRI(input.Object)
	} else {
		obj, err = rdf.NewLiteral(input.Object)
	}
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid object"})
		return
	}

	triple := rdf.Triple{Subj: subj, Pred: pred, Obj: obj}
	triples = append(triples, triple)

	c.JSON(http.StatusCreated, gin.H{"message": "Triple added successfully"})
}

func isValidIRI(s string) bool {
	_, err := rdf.NewIRI(s)
	return err == nil
}
