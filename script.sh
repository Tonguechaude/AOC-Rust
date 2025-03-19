#!/bin/bash

for i in $(seq -w 1 31); do
    project="day-$i"

    if [ ! -d "$project" ]; then
        cargo new "$project" && echo "Projet $project créé."
    else
        echo "Le projet $project existe déjà, passage au suivant."
        continue
    fi

    if cd "$project"; then
        [ -d .git ] && rm -rf .git && echo "Dossier .git supprimé dans $project."

        cd ..
    else
        echo "Erreur : Impossible d'entrer dans $project."
    fi
done

