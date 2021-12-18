(ns day12
  (:require [clojure.pprint :refer [pprint]])
  (:require [clojure.string :as str])
  (:require [clojure.tools.trace :refer [trace deftrace]]))

(defn build-adj [edges]
  (reduce
    (fn [adj [a b]]
      (as-> adj v
        ;; Don't add :start as a destination
        (if (= b :start) v (update v a (fnil conj #{}) b))
        (if (= a :start) v (update v b (fnil conj #{}) a))))
    {} edges))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (map #(str/split % #"-" 2))
    (map (partial mapv
           #(cond
              (= "start" %) (keyword "start")
              (= "end" %) (keyword "end")
              (= (str/upper-case %) %) (keyword "big" %)
              (= (str/lower-case %) %) (keyword "small" %))))
    (build-adj)))

(defn successors [adj node]
  (if (= :end node) #{} (adj node)))

(defn big? [k] (= (namespace k) "big"))
(defn small? [k] (= (namespace k) "small"))

(defn succ-filter [path]
  (fn [succ] (or (big? succ) (not (some #{succ} path)))))

(defn succ-filter* [path]
  (fn [succ]
    (or
      (big? succ)
      (not (some #{succ} path))
      (->> path
        (filter small?)
        (frequencies)
        (not-any? #(> (val %) 1))))))

(defn paths [adj path f]
  (let [head (peek path)]
    (->> (successors adj head)
      ;; can't visit smalls twice
      (filter (f path))
      (mapcat #(paths adj (conj path %) f))
      (cons path))))


(defn run [input f]
  (->> (paths input [:start] f)
    (filter #(= (peek %) :end))
    (count)))

(defn -main []
  (let [input (parse "12.txt")]
    (pprint input)
    (println "Part 1:" (run input succ-filter))
    (println "Part 2:" (run input succ-filter*))))
