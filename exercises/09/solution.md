
## Discussion

Bei kleinen Anzahlen von Elementen sind die beiden Datenstrukturen wie erwartet ziemlich gleich.
Bei grossen Anzahlen von Elementen ist die Linked List wie erwartet viel schneller da diese aufgrund des linearen Traversierens alle Funktionen in der Klasse O(1) hat.
Hingegen der Array hat Read und Write in der Klasse O(1) waehrend Insert und Delete in O(n) sind.

Komischerweisse sind meine Ergebnisse mit Instructionmix 10% konsistent gleich oder sogar schneller wie die mit 0%. Hier wird entweder ein Harmoniefall oder ein Programmierfehler vorliegen.

Die Ergebnisse am LCC sind ziemlich gleich mit den lokalen Ergebnissen nur mit einem Faktor von 250%-500% multipliziert.
