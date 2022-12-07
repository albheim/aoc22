(def day "day02")
(def test-data (slurp (reduce str ["data/" day ".test"])))
(def real-data (slurp (reduce str ["data/" day ".data"])))

(defn parse [input]
  ; Split into list commands in range 0-2
  (map (fn [line] 
         [(- (int (first line)) (int \A))
          (- (int (nth line 2)) (int \X))])
       (clojure.string/split-lines input)))

(defn play [opponent you]
  ; 0 is loss, 1 is draw, 2 is win
  (mod (- (inc you) opponent) 3))

(defn value [outcome]
  (* 3 outcome))

(defn run-a [games]
  ; Run each game and sum score
  (reduce + (map (fn [game]
                   (+ (value (apply play game)) (second game) 1))
                 games)))

(run-a (parse test-data))

(println "Part a:" (run-a (parse real-data)))

(defn move [opponent outcome]
  ; Find move that would result in outcome
  (mod (+ opponent outcome 2) 3))

(defn run-b [games]
  ; Run each game and sum score
  (reduce + (map (fn [game]
                   (+ (value (second game)) (apply move game) 1))
                 games)))

(run-b (parse test-data))

(println "Part b:" (run-b (parse real-data)))

