/// @TODO Translate in english
/// GS (Group separator)
/// Lorsqu’un champ est de longueur variable, qu’il n’a pas atteint sa longueur maximale et qu’il n’est pas
/// le  dernier  champ,  il  se  termine  par  le  caractère  de  contrôle  <GS>  (code  ASCII  29).  Le  champ  de
/// longueur variable et libre se termine par le caractère de contrôle <GS> lorsqu’il n’est pas le dernier
/// champ.
const MESSAGE_PART_SEPARATOR: char = '\u{001D}';
/// @TODO Translate in english
/// RS (Record separator)
const TRUNCATED_MESSAGE_PART_END: char = '\u{001E}';
