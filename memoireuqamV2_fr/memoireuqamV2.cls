%
% format LaTeX pour memoire et these de l'UQAM
% d'apres "Guide de presentation des memoires et theses"
% auteur: Gisele Legault
% collaborateurs: Stephane Poirier, Rene Ferland,
%                  Luc Belanger, Andre Lauzon
%                  Jerome Tremblay, Anthony Monnet, 
%                  Diego Maupomé, Guillaume Gagnon

% Modifications:
% Février 2023 (DM)
% Modifications majeures visant le rendu en format électronique.
%     Entre autres :
%      - recto uniquement avec marges égales
%      - numérotation au centre de la page
%      - pagination de toutes les pages sauf la page-titre du document
%      - police taille 11pt
%      - police carlito par défaut
%      - changements d'espacement avant et après les sous-titres et entre les paragraphes
%      - interligne simple dans les titres de figures et tableaux
%      - interligne 1.5 dans les notes de bas de page
%      - bibliographie : retrait négatif et changement de l'espace entre les références
%     - hyperliens sans couleur
%      - bug relié à l'usage d'hyperref qui renvoyaient les références vers la page titre réglé
%      - prise en charge de l'ajout de la bibliographie à la table des matières avec natbib
%      - mode projet de thèse ajouté \uqamprojet
%      - suppression des avertissements liés aux majuscules dans les titres (GG)
%      - environnements mathématiques : theorem, corollary, lemma, proposition, definition, proof
% De plus, modifications à l'exemple :
%      - fichiers de glossaires (notation, acronymes)
%      - exemples de schémas, graphiques
     
% Novembre 2018
% Modification de la mise en forme de la table des matières
% Chapitre, Annexe et leur titre inscrit sur la même ligne
% décembre 2017
% Modification de l'interligne avant et après une section, sous-section et sous-sous-section
% Modification alignement NON justifié pour la bibliographie
% janvier 2017
% Supprimé \bf dans le titre d'une figure et d'un tableau (caption)
% août 2015
% Modifié \@sect pour ne pas ajouter la section dans la table des matières
% lorsque le niveau de la section est supérieur à 2
% mai 2015
% Mise en forme correcte d'une section, sous-section etc. lorsque le texte est très long dans le
% corps du document: simple interligne et alignement à gauche
% Supprimer \large dans la page titre et les chapitres (\@makechapterhead et \@makeschapterhead)
% avril 2014
% modifier definition de \l@chapter pour éliminer le retrait
% janvier 2014
% definition de \setstretch, \doublespacing, doublespace si le package setspace n'est pas chargé
% décembre 2013
% ajout de \concentration pour la page titre
% ajout du mois et de l'année courante par défaut pour \degreeyear et \degreemonth
% aout 2013
% commande UpperRef defini en fonction du chargement du package hypperref
% ajout \matiere pour page titre  - Anthony Monnet
% vrification chargement package natbib. Si oui, utiliser la \bibitem de natbib
% sinon utiliser \bibitem de memoireuqamV2
% juillet 2022
% Ajouté des commandes du package setspace pour avoir double
% interligne dans la page titre
% avril 2012
% Remplacer \par par \\ dans la page titre pour avoir la même interligne partout
% Mis la liste des tableaux et la liste des figures en simple interligne
% juin 2011
% Ajouté Figure + page dans la liste des figures
% Ajouté Tableau + page dans la liste des tableaux
% Octobre 2011 - Gisèle Legault
% corrigé erreur avec la \paragraph et \subparagraph
% Novembre 2010 - tremblay.jerome@uqam.ca
%   Enlevé le gras dans les titres
%   Ne pas paginer les pages vides
% janvier 2007
% ajouté \evensidemargin pour impression recto-verso
% octobre 2004
%1) Mis les noms des sections en majuscules pour utiliser le package babel
%2) enlever definition \og et \fg (redefini par babel)
%3) ajouter \uqammemoire, \uqamthese et \uqamrapport pour la page titre

% Un de mes ajouts (Luc Belanger)
\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{memoireuqamV2}
              [juillet 2022
 Format LaTeX pour les memoires a l'UQAM.]

% Un autre ajout pour enlever le warning (Luc Belanger)
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{report}}
\ProcessOptions

\LoadClass{report}

\def\mystretch{1.5}             % interligne et demi


\def\baselinestretch{\mystretch}        % interligne et demi

\newlength{\simpleinter}
\setlength{\simpleinter}{\baselineskip}
\newlength{\doubleinter}
\setlength{\doubleinter}{1.5\baselineskip}

% changer pt à bp pour correspondre aux unités de la spéc MS Word
\setlength{\p@}{1bp}


%%%%%%%  Set up margins and formatting params %%%


%%%%%%%%%%%%%%%%%%%%%%
% Marges
%%%%%%%%%%%%%%%%%%%%%%
% Décalage gauche : 1" + \hoffset + \oddsidemargin

\setlength{\oddsidemargin}{-.04cm}      % 2.5cm left margin
\setlength{\topmargin}{-.04cm}           % 2.5cm top margin
\setlength{\textwidth}{16.59cm}          % 6.0in text - 2.5cm rt margin
\setlength{\textheight}{22.94cm} % Body ht for 2.5cm margins
\addtolength{\topmargin}{-\headheight}  % No header, so compensate
\addtolength{\topmargin}{-\headsep}     % for header height and separation
\setlength{\footnotesep}{6pt}
%%%%%%%  End of margins and formatting params %%%


% Novembre 2010 - Ne pas paginer les pages vides (tremblay.jerome@uqam.ca)
% Code for creating empty pages
% No headers on empty pages before new chapter
\def\cleardoublepage{\clearpage\if@twoside \ifodd\c@page\else
    \hbox{}
    \thispagestyle{empty}
    \newpage
    \if@twocolumn\hbox{}\newpage\fi\fi\fi}

\edef\partpdf#1\expandafter{\uppercase{#1}}


\AtBeginDocument{
   \@ifpackageloaded{hyperref}
   {\newcommand{\UpperRef}[1]{\texorpdfstring{\MakeUppercase{#1}}
                   { \expandafter\expandafter\partpdf\expandafter{#1} } }
    
   }
   {\newcommand{\UpperRef}[1]{\MakeUppercase{#1}}}
\@ifpackageloaded{hyperref}
{\hypersetup{hidelinks}}

\@ifpackageloaded{babel}
{\@ifpackagewith{babel}{french}
{\def\mois_alpha{\ifcase\month\or
Janvier\or F\'{e}vrier\or Mars\or Avril\or Mai\or Juin\or Juillet\or Ao\^{u}t\or Septembre\or Octobre\or Novembre\or D\'{e}cembre\fi}}
{\def\mois_alpha{\ifcase\month\or
January\or February\or March\or April\or May\or June\or
July\or August\or September\or October\or November\or December\fi}}
}
{\def\mois_alpha{\ifcase\month\or
January\or February\or March\or April\or May\or June\or
July\or August\or September\or October\or November\or December\fi}}


\@ifpackageloaded{natbib}
  {\relax
  \setlength\bibhang{1.27cm}   % retrait négatif avec natbib
   
\NewCommandCopy{\natbiblio}{\thebibliography}
\NewCommandCopy{\endnatbiblio}{\endthebibliography}

\renewenvironment{thebibliography}[1]
     {\phantomsection
     \addtocontents{toc}{\protect\vspace{1.5ex}}
          % Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
      %\addcontentsline{toc}{chapter}{\UpperRef{\bibname}}
      \addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{\bibname}}{\bibname}}
      \baselineskip=\simpleinter
      \raggedright % bibliographie non justifié à droite G.L. 13/12/2017
      \natbiblio \endnatbiblio
      }
      {}
  }
  {

\def\@citex[#1]#2{%
  \let\@citea\@empty
  \@cite{\@for\@citeb:=#2\do
    {\@citea\def\@citea{;\penalty\@m\ }%
     \edef\@citeb{\expandafter\@firstofone\@citeb}%
     \if@filesw\immediate\write\@auxout{\string\citation{\@citeb}}\fi
     \@ifundefined{b@\@citeb}{\mbox{\reset@font\bfseries ?}%
       \G@refundefinedtrue
       \@latex@warning
         {Citation `\@citeb' on page \thepage \space undefined}}%
       {\csname b@\@citeb\endcsname}}}{#1}}

\def\hang{\hangindent\parindent}
\def\@cite#1#2{({#1\if@tempswa , #2\fi})}

\def\@biblabel#1{(#1)\hfill}

\def\@lbibitem[#1]#2{%
    \@skiphyperreftrue
    \H@item[%
      \ifx\Hy@raisedlink\@empty
        \hyper@anchorstart{cite.#2\@extra@b@citeb}%
        \hyper@anchorend
      \else
        \Hy@raisedlink{%
          \hyper@anchorstart{cite.#2\@extra@b@citeb}\hyper@anchorend
        }%
      \fi
      \hfill
    ]%
    \@skiphyperreffalse
    \if@filesw
      \begingroup
        \let\protect\noexpand
        \immediate\write\@auxout{%
          \string\bibcite{#2}{#1}%
        }%
      \endgroup
    \fi
    \ignorespaces
  }%

\def\bibitem{\@ifnextchar[{\@lbibitem}{\@bibitem}}
\newdimen\bibindent
\bibindent=1.5em

%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%  BIBLIOGRAPHIE
%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\renewenvironment{thebibliography}[1]
     {\chapter*{\UpperRef{\bibname}
        \@mkboth{\UpperRef{\bibname}}{\UpperRef{\bibname}}}%
      \addtocontents{toc}{\protect\vspace{1.5ex}}
      % Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
      %\addcontentsline{toc}{chapter}{\UpperRef{\bibname}}
      \addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{\bibname}}{\bibname}}
      \baselineskip=\simpleinter
      \list{\@biblabel{\arabic{enumiv}}}%
          {\settowidth\labelwidth{\@biblabel{#1}}%
            \setlength{\leftmargin}{1.27cm} % retrait négatif sans natbib
            \setlength{\itemindent}{-1.27cm}
            \@openbib@code
            \setlength{\listparindent}{-\leftmargin}
            % espacement vertical
            \setlength{\itemsep}{0pt}
            \setlength{\parsep}{12pt}
            \usecounter{enumiv}%
            \let\p@enumiv\@empty
            \renewcommand\theenumiv{\arabic{enumiv}}}%
      \sloppy\clubpenalty4000
      \@clubpenalty \clubpenalty
      \widowpenalty4000%
      \sfcode`\.=\@m
      \raggedright} % bibliographie non justifié à droite G.L. 13/12/2017
     {\def\@noitemerr
       {\@latex@warning{Empty `thebibliography' environment}}%
      \endlist}

  }
}
%% Ajout des commandes setstrerch et doublespacing si
%% package setspace non chargé                       G.L. 23/01/2014
\AtBeginDocument{ \@ifpackageloaded{setspace}
   {\relax}
   {\newcommand{\restore@spacing}{%
    \par
    \vskip \parskip
    \vskip \baselineskip
  \endgroup
  \vskip -\parskip
  \vskip -\baselineskip
}

\newcommand{\setstretch}[1]{%
  \def\baselinestretch{#1}%
  \@currsize
}
\newcommand{\doublespacing}{%
  \setstretch {2}%  default
}

% double spacing is 2 x pt size
\newenvironment{doublespace}{%
  \begingroup
    \doublespacing
}{%
  \restore@spacing
}
}
}

% Espacer acronymes et notation dans la table des matières
\AtBeginDocument{
    \@ifpackageloaded{glossaries}
{
% Espacer les glossaires dans la table des matières
\renewcommand*{\@gls@toc}[2]{%
  \ifglstoc
    \addtocontents{toc}{\protect\vspace{1.5ex}}
    \ifglsnumberline
      \addcontentsline{toc}{#2}{\protect\numberline{}#1}%
    \else
      \addcontentsline{toc}{#2}{#1}%
    \fi
  \fi
}
}{}
}

% Redefine '/chapter' to always start on an odd page.
% Should make no difference in singleside mode.
%
\def\chapter{\cleardoublepage   % Starts new page.
   \global\@topnum\z@           % Prevents figures from going at top of page.
   \@afterindenttrue            % indent in first paragraph.  Change
   \secdef\@chapter\@schapter}  % to \@afterindentfalse to have none.

% If using the report style, use - instead of . in the figure number.

\newcommand\numerochap{\arabic{chapter}}
\renewcommand\thechapter                {\arabic{chapter}}
\renewcommand\thesection                {\thechapter.\arabic{section}}
\@ifundefined{thechapter}{}{\def\thefigure{\thechapter-\arabic{figure}}}

%% supprime \large dans le titre de chapitre   G.L. 07/05/2015
%  Changement de l'espacement après les titres et compensation pour les chapitres spéciaux (e.g. introduction, conclusion)  DM (2022/07)
\def\@makechapterhead#1{%
  {\parskip=0pt
   \parindent \z@ \raggedright \reset@font
    \ifnum \c@secnumdepth >\m@ne
         \centering
         \bfseries
         {\expandafter\MakeUppercase\expandafter\@chapapp{} \numerochap} \par
       \fi
     \centering {\MakeUppercase{#1}}\par
      \nobreak
  \vspace*{18bp}
}\parskip=18bp}

\def\@makeschapterhead#1{
   \parskip=0pt
   \begin{center}
   \bfseries
     {\MakeUppercase{ #1 }}\par 
   \end{center}
   \nobreak \vspace*{0 truecm}
   \parskip=18bp  }

%%%%%%%%%%%%%%%%
% FRANCISATION DES TERMES
%%%%%%%%%%%%%%%%

\renewcommand\contentsname{TABLE DES MATI\`ERES}
\renewcommand\listfigurename{LISTE DES FIGURES}
\renewcommand\listtablename{LISTE DES TABLEAUX}
\renewcommand\bibname{BIBLIOGRAPHIE}
\renewcommand\indexname{INDEX}
\renewcommand\figurename{Figure}
\renewcommand\tablename{Tableau}
\renewcommand\partname{PARTIE}
\renewcommand\chaptername{CHAPITRE}
\renewcommand\appendixname{ANNEXE}  % annexe
\renewcommand\abstractname{R\'ESUM\'E}

%%%%%%%%%%%%%%%%%%%
%% INDENTATION DES SECTIONS
%%%%%%%%%%%%%%%%%%%

%% supprime \large et ajoute simple interligne dans section et autre G.L. 07/05/2015
%% Modification des interlignes avant et après les section et autre G.L. 13/12/2017
%% IDEM DM (voir détails L356) (2022/07)
\setcounter{secnumdepth}{5}

\renewcommand\section{\@startsection{section}{1}%
                                      {\z@}%
                                      {12bp\@plus.2ex\@minus.2ex}%
                                  {0.01bp\@plus.2ex}% hax
             {\normalfont\normalsize\baselineskip=\simpleinter}
}%
\renewcommand\subsection{\@startsection{subsection}{2}{\z@}%
                                     {12bp\@plus .2ex \@minus .2ex}%
                                     {.01pt \@plus .2ex}%
                                    {\normalfont\normalsize\baselineskip=\simpleinter}
}
\renewcommand\subsubsection{\@startsection{subsubsection}{3}{\z@}%
                                     {12bp\@plus .2ex \@minus .2ex}%
                                     {.01pt \@plus .2ex}%
                                     {\normalfont\normalsize\baselineskip=\simpleinter}}

\renewcommand\paragraph{\@startsection{paragraph}{4}{\z@}%
                                    {12bp \@plus1ex \@minus.2ex}%
                                    {.01pt}%
                                    {\normalfont\normalsize\baselineskip=\simpleinter}}
\renewcommand\subparagraph{\@startsection{subparagraph}{5}{\z@}%
                                      {12bp \@plus1ex \@minus .2ex}%
                                      {.01pt}%
                                      {\normalfont\normalsize\baselineskip=\simpleinter}}

% Un espacement après paragraphe (parskip) est ajouté par défaut après un sous-titre.
% Dans notre cas l'espacement désiré après un titre est plus petit (12pt) que parskip (18pt).
% Alors que le champ afterskip permet d'ajouter un espacement supplémentaire après le titre, cet espace ne peut être négatif afin de ramener le total à 12pt, car cela déclenche un titre en ligne.
% Il est donc nécessaire d'ajouter cet espace négatif par un autre moyen.
\let\origsection\section
\renewcommand\section[1]{\@ifstar{\starsection{\origsection}{#1}}{\nostarsection{\origsection}{#1}}}

\let\origsubsection\subsection
\renewcommand\subsection[1]{\@ifstar{\starsection{\origsubsection}{#1}}{\nostarsection{\origsubsection}{#1}}}

\let\origsubsubsection\subsubsection
\renewcommand\subsubsection[1]{\@ifstar{\starsection{\origsubsubsection}{#1}}{\nostarsection{\origsubsubsection}{#1}}}

\let\origparagraph\paragraph
\renewcommand\paragraph[1]{\@ifstar{\starsection{\origparagraph}{#1}}{\nostarsection{\origparagraph}{#1}}}

\let\origsubparagraph\subparagraph
\renewcommand\subparagraph[1]{\@ifstar{\starsection{\origsubparagraph}{#1}}{\nostarsection{\origsubparagraph}{#1}}}

\newcommand\nostarsection[2]
{#1{#2}\sectionpostlude}
\newcommand\starsection[2]
{#1{#2}\sectionpostlude}

\newcommand\sectionpostlude{%
  \vspace{-6bp}
}



\markright{}
\pagestyle{myheadings}
\pagenumbering{arabic}      % Arabic page numbers
%%%%%%% pas de renfoncement au début d'un paragraphe
\parindent=0ex

% \parskip=2ex

\pagestyle{plain}

%%%%%%%%%%%%
%%
%%  FIGURES ET TABLES
%%
%%%%%%%%%%%%


\addtocontents{lof}{\protect\addvspace{.5ex}}%

\setlength\abovecaptionskip{10\p@}
\setlength\belowcaptionskip{5\p@}

\long\def\@makecaption#1#2{%
  \vskip\abovecaptionskip\baselineskip=\simpleinter
  \sbox\@tempboxa{{#1} #2}   % supprime \bf dans \sbox\@tempboxa{{\bf #1} #2} G.L. 16/01/2017
  \ifdim \wd\@tempboxa >\hsize
    {#1} #2\par              % supprime \bf dans {\bf #1} #2\par G.L. 16/01/2017
  \else
    \global \@minipagefalse
    \hb@xt@\hsize{\hfil\box\@tempboxa\hfil}%
  \fi
  \vskip\belowcaptionskip}


\renewcommand\thefigure{{\thechapter.\@arabic\c@figure}}
\def\fps@figure{tbp}
\def\ftype@figure{1}
\def\ext@figure{lof}
\def\fnum@figure{{\figurename~}\thefigure}
\renewenvironment{figure}
               {\@float{figure}}
               {\end@float}
\renewenvironment{figure*}
               {\@dblfloat{figure}}
               {\end@dblfloat}

\renewcommand\thetable{{\thechapter.\@arabic\c@table}}
\def\fps@table{tbp}
\def\ftype@table{2}
\def\ext@table{lot}
\def\fnum@table{{\tablename~}\thetable}
\renewenvironment{table}
               {\@float{table}}
               {\end@float}
\renewenvironment{table*}
               {\@dblfloat{table}}
               {\end@dblfloat}

%%%%%%%%%%%%%%%%%
%%
%% TABLE DES MATIERES
%%
%%%%%%%%%%%%%%%%%

\renewcommand\@dotsep{1}

\newcommand\l@specialchapitre {\@dottedtocline{1}{0em}{0em}}

\renewcommand\tableofcontents{%
    \baselineskip=1.5\simpleinter
    \if@twocolumn
      \@restonecoltrue\onecolumn
    \else
      \@restonecolfalse
    \fi
    \chapter*{\expandafter\MakeUppercase\expandafter{\contentsname}}
    \parskip=0ex
    \vspace*{-1.5ex}
    \@starttoc{toc}%
    \noindent
    \if@restonecol\twocolumn\fi
    \baselineskip=\doubleinter
    \parskip=2ex
    
    \thispagestyle{plain}
}

\renewcommand\l@chapter[2]{%
  \ifnum \c@tocdepth >\m@ne
    \addpenalty{-\@highpenalty}%
    \vskip 1.0em \@plus\p@
    \setlength\@tempdima{1.5em}%
    \begingroup
      \parindent \z@ \rightskip \@pnumwidth
      \parfillskip -\@pnumwidth
      \nopagebreak
      \leavevmode %\bfseries
      \hskip -\leftskip
%      \@dottedtocline{1}{0em}{2.3em}
      #1\nobreak\hfil \nobreak\hbox to\@pnumwidth{\hss #2}\par
      \penalty\@highpenalty
    \endgroup
  \fi}
\def\@chapter[#1]#2{\ifnum \c@secnumdepth >\m@ne
                         \refstepcounter{chapter}%
                         \typeout{\@chapapp\space\thechapter.}%
                       % Chapitre et titre sur la meme ligne G.L. 23/11/2018
                         \addtocontents{toc}{\protect\vspace{1.5ex}}
                         
                         % Fix for both warnings: - G.G. 20221125
                         % Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' 
                         % Token not allowed in a PDF string (Unicode):(hyperref) \hskip 5.49545pt(hyperref) replaced by space
                         %\addcontentsline{toc}{chapter}{\UpperRef{\@chapapp\protect\hskip 0.5em \numerochap} \protect\hskip 0.5em \UpperRef{#1}} 
                         \addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{\@chapapp\protect\hskip 0.5em \numerochap} \protect\hskip 0.5em \UpperRef{#1}}
                                                                       {\@chapapp\protect\space \numerochap \protect\space #1}} 

                     \else
                      \addcontentsline{toc}{chapter}{#1}
                    \fi
                    \chaptermark{#1}%
                    \if@twocolumn
                      \@topnewpage[\@makechapterhead{#2}]%
                    \else
                      \@makechapterhead{#2}%
                      \@afterheading
                    \fi
                    }


\def\@sect#1#2#3#4#5#6[#7]#8{\ifnum #2>\c@secnumdepth
      \let\@svsec\@empty \else
    \refstepcounter{#1} \protected@edef\@svsec{\@seccntformat{#1}\hskip 1em}%
  \fi
  \@tempskipa #5\relax
  \ifdim \@tempskipa>\z@
    \begingroup #6{\relax
        \@hangfrom{\hskip #3\relax\@svsec}\interlinepenalty \@M #8\@@par}%
    \endgroup
    \csname #1mark\endcsname{#7}%
	\ifnum #2>\tw@ \else
    \addtocontents{toc}{\protect\vspace{1.5ex}} %ne pas inserer si niveau de section > 2
    \addcontentsline
    {toc}{#1}{\ifnum #2>\c@secnumdepth \else
        \protect\numberline{\csname the#1\endcsname}%
      \fi #7} \fi \else
    \def\@svsechd{%
      #6{\hskip #3\relax
       \@svsec #8}\csname #1mark\endcsname
        {#7} \addcontentsline{toc}{#1}{%
        \ifnum #2>\c@secnumdepth \else
          \protect\numberline{\csname the#1\endcsname}%
        \fi
        #7}}
  \fi
  \@xsect{#5}}

%% modification indentation de \@chapter   G.L. 07/05/2015

\renewcommand\l@chapter      {\@dottedtocline{0}{0em}{0em}}
\renewcommand\l@section      {\@dottedtocline{1}{0em}{2.3em}}
\renewcommand\l@subsection   {\@dottedtocline{2}{2.3em}{3.2em}}
\renewcommand\l@subsubsection{\@dottedtocline{3}{7.0em}{4.1em}}

%%%%%%%%%%%%%%%%%
%%
%% ENVIRONNEMENT APPENDICE
%%
%%%%%%%%%%%%%%%%%

\def\appendix{\par
  \setcounter{chapter}{0}%
  \setcounter{section}{0}%
  
  % Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
  % modifie \MakeUppercase par \UpperRef   G.L. 28/08/2013
  %\def\@chapapp{\UpperRef{\appendixname}}
  \def\@chapapp{\texorpdfstring{\UpperRef{\appendixname}}{\appendixname}}
  
  \renewcommand\numerochap{\Alph{chapter}}
  \def\thechapter{\Alph{chapter}}}

%%%%%%%%%%%%%%%%%
%%
%% LISTE DES FIGURES
%%
%%%%%%%%%%%%%%%%%

% hack pour avoir le mot Figure devant chaque numéro dans la liste des figures DM (2022/07)
\renewcommand\l@figure {\@dottedtocline{0}{0em}{2.3em}}
\renewcommand\listoffigures{%
{
    \baselineskip=\simpleinter
    \if@twocolumn
      \@restonecoltrue\onecolumn
    \else
      \@restonecolfalse
    \fi
  % modifie \MakeUppercase par \UpperRef   G.L. 28/08/2013
    \chapter*{\UpperRef{\listfigurename}
      \@mkboth{\UpperRef{\listfigurename}}%
              {\UpperRef{\listfigurename}}}%
    \addtocontents{toc}{\protect\vspace{1.5ex}}
    
    % Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
    %\addcontentsline{toc}{chapter}{\UpperRef{\listfigurename}}
    \addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{\listfigurename}}{\listfigurename}}
    
    \let\oldnumberline\numberline%
    \renewcommand{\numberline}{\figurename~\oldnumberline}
    \@starttoc{lof}%
    \if@restonecol\twocolumn\fi
    }
    \thispagestyle{plain}
}

%%%%%%%%%%%%%%%%%
%%
%% LISTE DES TABLEAUX
%%
%%%%%%%%%%%%%%%%%

% hack pour avoir le mot Tableau devant chaque numéro dans la liste des figures DM (2022/07)
\renewcommand\l@table {\@dottedtocline{0}{0em}{2.3em}}
\renewcommand\listoftables{%
{
    \baselineskip=\simpleinter

    \if@twocolumn
      \@restonecoltrue\onecolumn
    \else
      \@restonecolfalse
    \fi
      % modifie \MakeUppercase par \UpperRef   G.L. 28/08/2013
    \chapter*{\UpperRef{\listtablename}
      \@mkboth{\UpperRef{\listtablename}}{\UpperRef{\listtablename}}}%
    \addtocontents{toc}{\protect\vspace{1.5ex}}
    
    % Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
    %\addcontentsline{toc}{chapter}{\UpperRef{\listtablename}}
    \addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{\listtablename}}{\listtablename}}
    
    \let\oldnumberline\numberline%
    \renewcommand{\numberline}{\tablename~\oldnumberline}
    \@starttoc{lot}%
    \if@restonecol\twocolumn\fi
    }
    
    \thispagestyle{plain}
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% Footnote
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

% Changement de l'interligne, l'espacement et le retrait DM (2022/07) 
\setlength{\footnotesep}{\simpleinter}

\renewcommand{\@makefntext}[1]%
  {\setlength{\parindent}{0em}%
   \makebox[.8em][l]{\@makefnmark}#1}

\renewcommand{\footnoterule}{
 \noindent
 \rule{4cm}{0.4pt}
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% Environnement Resume
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\renewenvironment{abstract}{%
% modifie \MakeUppercase par \UpperRef   G.L. 28/08/2013
  \chapter*{\UpperRef{\abstractname}}
  \addtocontents{toc}{\protect\vspace{1.5ex}}
  
  % Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
  %\addcontentsline{toc}{chapter}{\UpperRef{\abstractname}}
  \addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{\abstractname}}{\abstractname}}

  \baselineskip=\simpleinter
  \thispagestyle{plain} %  pagination centrée
  }

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% Environnement Introduction
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\newenvironment{introduction}{%
\chapter*{INTRODUCTION}
\addtocontents{toc}{\protect\vspace{1.5ex}}

% Fix for warning: Token not allowed in a PDF string (Unicode):(hyperref) removing `\uppercase' - G.G. 20221125
%\addcontentsline{toc}{chapter}{\UpperRef{INTRODUCTION}}
\addcontentsline{toc}{chapter}{\texorpdfstring{\UpperRef{INTRODUCTION}}{INTRODUCTION}}
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% Environnement CONCLUSION
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\newenvironment{conclusion}{%
 \chapter*{CONCLUSION}
 \addtocontents{toc}{\protect\vspace{1.5ex}}
 \addcontentsline{toc}{chapter}{CONCLUSION}
}


%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
%  Sections mathématiques
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\newtheorem{theorem}{Théorème}[chapter]
\newtheorem{corollary}[theorem]{Corollaire}
\newtheorem{lemma}[theorem]{Lemme}
\newtheorem{proposition}[theorem]{Proposition}
\newtheorem{definition}[theorem]{Définition}
\AtBeginDocument{
\@ifpackageloaded{amsfonts}{
\newcommand{\qed}{$\square$}
}{
\newcommand{\qed}{\textit{QED}.}
}
\newenvironment*{proof}{\medskip\textit{Preuve.}~\itshape\ignorespaces}{\qed}
}


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% PAGE TITRE
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%\newif\ifisthesis
%\global\isthesistrue


\def\degree#1{\setbox0\hbox{#1}
\def\@degree{#1}}
\def\degreemonth#1{\gdef\@degreemonth{#1}}
\def\degreeyear#1{\gdef\@degreeyear{#1}}
% Ajout de \matiere  A.M. 28/08/2013
\def\matiere#1{\gdef\@matiere{#1}}
% matiere par defaut = mathematiques
\matiere{MATH\'EMATIQUES}
% Ajout de \concentration  G.L. 17/12/2013
\def\concentration#1{\gdef\@concentration{#1}}
\concentration{}
\degreeyear{\the\year}
\degreemonth{\mois_alpha}

\def\thesetype{TH\`ESE}
\def\theseprog{DU DOCTORAT EN \expandafter\MakeUppercase\expandafter{\@matiere}}
\def\accord{E}

\def\uqamthese{
   %\isthesistrue
   \gdef\thesetype{TH\`ESE}
   \gdef\theseprog{DU DOCTORAT EN \expandafter\MakeUppercase\expandafter{\@matiere}}
   \gdef\accord{E}
}

% projet (proposal) de thèse   DM (2022/07)
\def\uqamprop{
   %\isthesistrue
   \gdef\thesetype{PROJET DE TH\`ESE}
   \gdef\theseprog{DU DOCTORAT EN \expandafter\MakeUppercase\expandafter{\@matiere}}
}

\def\uqammemoire{
   %\isthesisfalse
   \gdef\thesetype{M\'EMOIRE}
   \gdef\theseprog{DE LA MA\^ITRISE EN \expandafter\MakeUppercase\expandafter{\@matiere}}
   \gdef\accord{}
}

\def\uqamrapport{
   %\isthesisfalse
   \gdef\thesetype{RAPPORT DE RECHERCHE}
   \gdef\theseprog{DE LA MA\^ITRISE EN \expandafter\MakeUppercase\expandafter{\@matiere}}
   \gdef\accord{}
}




\def\maketitle{\begin{titlepage}
\begin{doublespace}
\begin{center}
%% suppression \large dans la page titre G.L. 07/05/2015


UNIVERSIT\'E DU QU\'EBEC \`A MONTR\'EAL\\
\vfill


{\MakeUppercase{\@title}}\\
\vfill

\thesetype
\\
PR\'ESENT\'E\accord \\
COMME EXIGENCE PARTIELLE\\
\theseprog\\
\ifx\@concentration\empty
\else
\expandafter\MakeUppercase\expandafter{\@concentration}\\
\fi
\vfill
PAR\\
{\MakeUppercase{\expandafter{\@author}}}
\\
\vfill
\MakeUppercase{\expandafter{\@degreemonth} \@degreeyear}
\\
\end{center}
\end{doublespace}
\end{titlepage}
}
